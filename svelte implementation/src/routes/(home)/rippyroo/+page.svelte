<script>
	// If you've redeployed the wasm/schema for the Rippy Roo blueprint, you will need to update the component addresses and resource addresses that are "hard" coded below.
	let vaultQuery;
	let vaultAddress = 'refresh to load';
	let scoreCards;
	let firstScoreCard;
	let scoreCardUpdate = 'refresh to load';
	let refreshScoreCardData;
	let NewScoreData;
	let userScore = '0';
	let bestScore = 0;

	import {
		RadixDappToolkit,
		ManifestBuilder // I haven't used the ManifestBuilder tool, instead I just used raw manifests - either options works
	} from '@radixdlt/radix-dapp-toolkit';

	const dAppId = 'account_tdx_22_1prd6gfrqj0avlyxwldgyza09fp7gn4vjmga7clhe9p2qv0qt58'; // linked to the .well-known/radix.json file

	let accountAddress = 'connect wallet to load'; //: string // User account address

	let userName = 'Connect'; // this variable will be set to the user's persona once they connect

	const rdt = RadixDappToolkit(
		{
			dAppDefinitionAddress: dAppId,
			dAppName: 'RippyMachine'
		},
		(requestData) => {
			requestData({
				accounts: {
					quantifier: 'atLeast',
					quantity: 1
				},
				persona: {
					quantifier: 'atLeast',
					quantity: 1
				}
			}).map(({ data: { accounts, persona } }) => {
				// add accounts and connected persona as a variable
				console.log('account data: ', accounts);
				console.log(persona.label);
				userName = persona.label;
				accountAddress = accounts[0].address;
			});
		},
		{
			networkId: 12
		}
	);
	console.log('dApp Toolkit: ', rdt);

	// There are four classes exported in the Gateway-SDK These serve as a thin wrapper around the gateway API
	// API docs are available @ https://betanet-gateway.redoc.ly/
	import {
		TransactionApi,
		StateApi,
		StatusApi,
		StreamApi
	} from '@radixdlt/babylon-gateway-api-sdk';

	// Instantiate Gateway SDK
	const transactionApi = new TransactionApi();
	const stateApi = new StateApi(); // I don't use these other API end points, but here for reference
	const statusApi = new StatusApi(); // I don't use these other API end points, but here for reference
	const streamApi = new StreamApi(); // I don't use these other API end points, but here for reference

	// defining headers for the various calls made to the RCNET gateway.
	var myHeaders = new Headers();
	myHeaders.append('Content-Type', 'application/json');

	// this async function gets the metadata of a user's scorecard

	async function updateCard() {
		var raw = JSON.stringify({
			address: accountAddress,
			resource_address: 'resource_tdx_c_1qf9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqsj4jhc4'
		});

		var requestOptions = {
			method: 'POST',
			headers: myHeaders,
			body: raw,
			redirect: 'follow'
		};

		// first we find the vault that the NFT is held in within the user's wallet

		vaultQuery = await fetch(
			'https://rcnet.radixdlt.com/state/entity/page/non-fungible-vaults/',
			requestOptions
		)
			.then((response) => response.text())
			.then((result) => (vaultQuery = JSON.parse(result)))
			.catch((error) => console.log('error', error));
		vaultAddress = vaultQuery.items[0].vault_address;
		console.log(vaultAddress);

		var raw = JSON.stringify({
			address: accountAddress,
			resource_address: 'resource_tdx_c_1qf9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqsj4jhc4',
			vault_address: vaultAddress
		});

		var requestOptions = {
			method: 'POST',
			headers: myHeaders,
			body: raw,
			redirect: 'follow'
		};

		// Once we have the vault address, we can get the id of the user's NFT (in this case a UUID)
		// To do: currently a user can get as many score card NFTs as they want and the method below just gets the most recent NFT scorecard they've minted.
		// Either users should be able to select which scorecard they want to use, or the scrypto method for minting a card should first check if they own one already. - both are easy to implement, depending on use-case.

		scoreCards = await fetch(
			'https://rcnet.radixdlt.com/state/entity/page/non-fungible-vault/ids',
			requestOptions
		)
			.then((response) => response.text())
			.then((result) => (firstScoreCard = JSON.parse(result)))
			.catch((error) => console.log('error', error));

		scoreCardUpdate = firstScoreCard.items[0].non_fungible_id;
		console.log(scoreCardUpdate);

		var raw = JSON.stringify({
			resource_address: 'resource_tdx_c_1qf9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqsj4jhc4',
			non_fungible_ids: [scoreCardUpdate]
		});

		var requestOptions = {
			method: 'POST',
			headers: myHeaders,
			body: raw,
			redirect: 'follow'
		};

		// Then we can get the metadata of that specific NFT

		refreshScoreCardData = await fetch(
			'https://rcnet.radixdlt.com/state/non-fungible/data',
			requestOptions
		)
			.then((response) => response.text())
			.then((result) => (NewScoreData = JSON.parse(result)))
			.catch((error) => console.log('error', error));

		userScore = NewScoreData.non_fungible_ids[0].mutable_data.raw_json.elements[4].value;
		console.log(userScore);
	}

	// This method call mints a new Score Card and sends it to the user

	async function methodCall() {
		let manifest =
			`CALL_METHOD
    Address("component_tdx_c_1qd9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqs86hj03")
    "mint_score_card"
    "` +
			userName +
			`"
    "` +
			accountAddress +
			`";

 CALL_METHOD
    Address("` +
			accountAddress +
			`")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");`;

		console.log('mint_score_card manifest: ', manifest);

		// Send manifest to extension for signing
		const result = await rdt.sendTransaction({
			transactionManifest: manifest,
			version: 1
		});

		if (result.isErr()) throw result.error;

		console.log('mint_score_card Result: ', result);

		// Fetch the transaction status from the Gateway SDK
		let status = await transactionApi.transactionStatus({
			transactionStatusRequest: {
				intent_hash_hex: result.value.transactionIntentHash
			}
		});
		console.log('mint_score_card TransactionAPI transaction/status: ', status);

		// fetch commit reciept from gateway api
		let commitReceipt = await transactionApi.transactionCommittedDetails({
			transactionCommittedDetailsRequest: {
				transaction_identifier: {
					type: 'intent_hash',
					value_hex: result.value.transactionIntentHash
				}
			}
		});
		console.log('mint_score_card Committed Details Receipt', commitReceipt);
	}

	// this function allows the user to update their NFT with their new high score - it also sends back some ROO tokens.  (Even though the NFT updates with the new high score, I also provide some to track the high score just because the developer wallet doesn't show metadata yet)
	// create_proof by ids passes the NFT:ID
	// pop this proof from the Auth zone in order to pass the proof as an argument in the next method
	// pass as arguments a decimal number linked to the game's "best score" variable and the proof - this method will then update the metadata of the NFT and return some ROO tokens.
    // Obviously the system is not ideal - anyone could send a raw transaction and put any new number they want for their highscore. Could be resolved using an api in the blueprint or admin rights, but this is just an easy demo for fun.

	async function updateConfirm() {
		let manifest =
			`CALL_METHOD
        Address("` +
			accountAddress +
			`")
        "create_proof_by_ids" 
        Address("resource_tdx_c_1qf9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqsj4jhc4")
        Array<NonFungibleLocalId>(NonFungibleLocalId("` +
			scoreCardUpdate +
			`"));
    POP_FROM_AUTH_ZONE
        Proof("proof1");
    CALL_METHOD
        Address("component_tdx_c_1qd9velelj33fnqwc2w5wmqa03ewtkms45mfvx6tywfqs86hj03")
        "claim_high_score"
        Proof("proof1")
        Decimal("` +
			bestScore +
			`");
    CALL_METHOD
        Address("` +
			accountAddress +
			`")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP");`;

		console.log('claim_high_score manifest: ', manifest);

		// Send manifest to extension for signing
		const result = rdt.sendTransaction({
			transactionManifest: manifest,
			version: 1
		});

		console.log('High Score getMethods Result: ', result);
	}

	// ################################################################################################################

	// All of the following code is a clone of Flappy Bird, with some slight modifications.
	// I do not explain how this works - in any case, it's not great, I'm no game dev.

	// ################################################################################################################

	let w = window.innerWidth;
	let paintAble = true;
	if (w < 1100) {
		paintAble = false;
	}

	if (!window.requestAnimationFrame) {
		window.requestAnimationFrame =
			window.mozRequestAnimationFrame || window.webkitRequestAnimationFrame;
	}

	let gravity = 1.1;
	let speed = 6;
	let jump = -19.5;
	let topRefresh = 5;
	let bottomRefresh = 15;

	let fps = '';
	var t = [];

	function animate(now) {
		t.unshift(now);
		if (t.length > 10) {
			var t0 = t.pop();
			fps = Math.floor((1000 * 10) / (now - t0));
		}

		window.requestAnimationFrame(animate);
		if (fps >= 80) {
			gravity = 0.3;
			speed = 3;
			jump = -10.5;
			bottomRefresh = 30;
			topRefresh = 10;
		}
	}

	window.requestAnimationFrame(animate);

	let show = 'block';

	function start() {
		show = 'none';
		const canvas = document.getElementById('canvas');
		const ctx = canvas.getContext('2d');
		const img = new Image();
		img.src = './RippyBird2.png';

		// general settings
		let gamePlaying = false;

		const size = [51, 36];

		const cTenth = canvas.width / 10;
		let pipes;
		let index = 0,
			flight,
			flyHeight,
			currentScore,
			pipe;

		// pipe settings
		const pipeWidth = 78;
		const pipeGap = 270;
		const pipeLoc = () =>
			Math.random() * (canvas.height - (pipeGap + pipeWidth) - pipeWidth) + pipeWidth;

		const setup = () => {
			currentScore = 0;
			flight = jump;

			// set initial flyHeight (middle of screen - size of the bird)
			flyHeight = canvas.height / 2 - size[1] / 2;

			// setup first 3 pipes
			pipes = Array(3)
				.fill()
				.map((a, i) => [canvas.width + i * (pipeGap + pipeWidth), pipeLoc()]);
		};

		const render = () => {
			// make the pipe and bird moving
			index++;

			// ctx.clearRect(0, 0, canvas.width, canvas.height);

			// background first part
			ctx.drawImage(
				img,
				0,
				0,
				canvas.width,
				canvas.height,
				-((index * (speed / 2)) % canvas.width) + canvas.width,
				0,
				canvas.width,
				canvas.height
			);
			// background second part
			ctx.drawImage(
				img,
				0,
				0,
				canvas.width,
				canvas.height,
				-(index * (speed / 2)) % canvas.width,
				0,
				canvas.width,
				canvas.height
			);

			// pipe display
			if (gamePlaying) {
				pipes.map((pipe) => {
					// pipe moving
					pipe[0] -= speed;

					// top pipe
					ctx.drawImage(
						img,
						432,
						588 - pipe[1],
						pipeWidth,
						pipe[1],
						pipe[0],
						0,
						pipeWidth,
						pipe[1]
					);
					// bottom pipe
					ctx.drawImage(
						img,
						432 + pipeWidth,
						108,
						pipeWidth,
						canvas.height - pipe[1] + pipeGap,
						pipe[0],
						pipe[1] + pipeGap,
						pipeWidth,
						canvas.height - pipe[1] + pipeGap
					);

					// give 1 point & create new pipe
					if (pipe[0] <= -pipeWidth) {
						currentScore++;
						// check if it's the best score
						bestScore = Math.max(bestScore, currentScore);

						// remove & create new pipe
						pipes = [
							...pipes.slice(1),
							[pipes[pipes.length - 1][0] + pipeGap + pipeWidth, pipeLoc()]
						];
						console.log(pipes);
					}

					// if hit the pipe, end
					if (
						[
							pipe[0] <= cTenth + size[0],
							pipe[0] + pipeWidth >= cTenth,
							pipe[1] > flyHeight || pipe[1] + pipeGap < flyHeight + size[1]
						].every((elem) => elem)
					) {
						gamePlaying = false;
						setup();
					}
				});
			}
			// draw bird
			if (gamePlaying) {
				ctx.drawImage(
					img,
					432,
					Math.floor((index % bottomRefresh) / topRefresh) * size[1],
					...size,
					cTenth,
					flyHeight,
					...size
				);
				flight += gravity;
				flyHeight = Math.min(flyHeight + flight, canvas.height - size[1]);
				ctx.fillText(`Score : ${currentScore}`, 270, 45);
				ctx.font = 'bold 40px w95fa';
			} else {
				ctx.drawImage(
					img,
					432,
					Math.floor((index % bottomRefresh) / topRefresh) * size[1],
					...size,
					canvas.width / 2 - size[0] / 2,
					flyHeight,
					...size
				);
				flyHeight = canvas.height / 2 - size[1] / 2;
				// text accueil
				ctx.fillText(`Best Score : ${bestScore}`, 105, 245);
				ctx.fillText('Up Key to Rip & Roo', 58, 535);
				ctx.font = 'bold 40px w95fa';
			}

			// tell the browser to perform anim
			window.requestAnimationFrame(render);
		};

		// launch setup
		setup();
		img.onload = render;

		// start game

		document.onkeydown = checkKey;

		function checkKey(e) {
			e = e || window.event;

			if (e.keyCode == '38') {
				flight = jump;
				gamePlaying = true;
			}
		}
	}

	let updateModal = false;
	let InstructionsText = 'Read Instructions';

	function openModal() {
		updateModal = !updateModal;
	}

	let instructions = false;

	function openInstructions() {
		instructions = !instructions;
		if (instructions) {
			InstructionsText = 'Close Instructions';
		} else {
			InstructionsText = 'Read Instructions';
		}
	}
</script>

{#if paintAble === true}
	<div class="connectBtn">
		<radix-connect-button />
	</div>
	<div style="width: 100%; height: 100%; margin-left: auto; margin-right: auto;" class="windowsBox">
		<div class="grid">
			<div style="width: 100%;">
				<div>
					<p
						style="margin-left: 40px; font-weight: bold; font-size: 90px; color: white !important; margin-bottom: 0; line-height: 30px; text-shadow: 2px 2px #ff5e00;"
					>
						RIPPA'
					</p>
					<p
						style="margin-left: 40px; font-weight: bold; font-size: 90px; color: white !important; margin-bottom: 0; text-shadow: 2px 2px #ff5e00;"
					>
						ROO
					</p>
					<p style="margin-left: 40px; font-weight: bold; font-size: 22px;">
						Press the up arrow key to rippa' roo
					</p>
					<div
						class="windowsBox"
						style="width: 67%; height: fit-content; margin-left: 80px; margin-right: auto;"
					>
						<div class="titlebar">
							<span>{userName} Score Card</span>
						</div>
						<div style="text-align: center;">
							<button class="start" style="width: 100%;" on:click={updateCard}>Refresh</button>
						</div>
						<div style="padding-left: 5px; padding-right: 5px;">
							<p
								style="font-weight: bold; font-size: 34px; color: gold; text-shadow: 2px 2px #ff5e00; text-align: center;"
							>
								High Score: {userScore}
							</p>

							<p style="margin: 0;">NFT ID:</p>
							<p style="margin: 0;"><b>{scoreCardUpdate}</b></p>
						</div>
					</div>
				</div>
			</div>
			<div class="container" style="width: 100%;">
				<canvas class="content" style="background: black;" id="canvas" width="431" height="716" />
				<div
					on:click={start}
					on:keydown={start}
					style="display: {show};width: 431px; text-align: center;"
					class="overlay"
				>
					<button
						style="display: {show}; width: 20%; height: 40px; margin-left: auto; margin-right: auto; margin-top: 330px"
						>Start</button
					>
				</div>
			</div>

			<div style="height: 100%;">
				<main style="text-align: center; margin-top: 100px;">
					<div>
						<h1 style="font-weight: bold; color: black; margin-right: 20px;">
							Record your high score on the RCNET ledger
						</h1>
						<button class="start" on:click={openInstructions}>{InstructionsText}</button>
						{#if instructions}
							<div style="color: black;">
								<p style="text-align: left; margin-right: 20px;">
									Step 1. Download the Babylon Developer Wallet and Chrome Extension
								</p>
								<p style="text-align: left; margin-right: 20px;">
									Step 2. Connect your wallet and choose a persona
								</p>
								<p
									style="text-align: left; margin-right: 20px; font-weight: bold; margin-bottom: 4px; margin-top: 8px;"
								>
									Step 3. Claim your NFT Score Card
								</p>
								<p
									style="text-align: left; margin-right: 20px; font-weight: bold; margin-bottom: 10px; margin-top: 8px;"
								>
									Step 4. Get a new high score and record it on your NFT
								</p>
								<p style="text-align: left; margin-right: 20px;">
									As NFT metadata does not show in the developer wallet, you'll also recieve ROO
									tokens to keep track of your best score. You can also connect your wallet and
									refresh the NFT score card on the left to check your best score
								</p>
								<p style="text-align: left; margin-right: 20px;">
									Trouble connecting or recording your score? Disconnect your wallet, refresh the
									page, open and close your mobile wallet.
								</p>
								<p style="text-align: left; margin-right: 20px;">
									*This is just a demo for fun. NFTs/Tokens are worthless and may be wiped at any
									time. It's easy to cheat, I'm no game dev. May contain bugs.*
								</p>
							</div>
						{/if}
						{#if !instructions}
							<div style="margin-top: 40px">
								{#if userName != 'Connect'}
									<div
										style="background: green; width: 10px; height: 10px; border-radius: 100px; margin-left: auto; margin-right: auto;"
									/>
									<p>Welcome {userName}!</p>
								{/if}
								{#if userName === 'Connect'}
									<div
										style="background: red; width: 10px; height: 10px; border-radius: 100px; margin-left: auto; margin-right: auto;"
									/>
									<p>Please reconnect your wallet</p>
								{/if}
							</div>
							<div style="color: black; margin-top: 10px;">
								<p
									style="text-align: left; margin-right: 20px; font-weight: bold; margin-bottom: 4px; margin-top: 8px; font-size: 20px; text-align: center;"
								>
									Claim your NFT Score Card
								</p>
								<button class="start" on:click={methodCall}>Get Score Card</button>

								<p
									style="text-align: left; margin-right: 20px; font-weight: bold; margin-bottom: 4px; margin-top: 8px; font-size: 20px; text-align: center;"
								>
									Get a new high score and update your NFT score card
								</p>
								<button class="start" on:click={updateCard} on:click={openModal}
									>Update Score</button
								>
								{#if updateModal}
									<div class="windowsBox">
										<div
											style="padding: 20px; z-index: 100; width: 100%; height: fit-content; display: grid; grid-template-columns: 60% 40%;"
										>
											<div>
												<p style="margin: 10px 0 0 0;"><b>Update Your Score Card</b></p>
												<p><b>New Score: {bestScore}</b></p>
											</div>
											<div style="display: flex; align-items: center; height: 100%;">
												<button class="start" on:click={updateConfirm}>Send Update</button>
											</div>
										</div>
										<p style="margin: 0;">NFT will only update if new score is higher</p>
										<p style="margin: 0;">
											Check current score by refreshing the score card on the left
										</p>
									</div>
								{/if}
							</div>
						{/if}
					</div>
				</main>
			</div>
		</div>
	</div>
{/if}
{#if paintAble === false}
	<div style="width: 100%; text-align: center;">
		<img alt="" src="Notavailable.png" style="margin-left: auto; margin-right: auto;" />
	</div>
{/if}

<style>
	.titlebar {
		background: #060084;
		height: fit-content;
		font-size: 100%;
		line-height: 100%;
	}

	.titlebar span {
		margin: 0;
		padding: 2px 0px 2px 5px;
		color: white;
		font-weight: bold;
		font-size: 1rem;
		line-height: 1.3rem;
	}
	.windowsBox {
		box-shadow: 0 4px 8px 0 rgb(0 0 0 / 20%) !important;
		transition: 0.3s;

		padding: 4px !important;
		font-size: 1rem;
		border-style: solid !important;
		border-width: 2px !important;
		border-color: #dfdfdf #0a0a0a #0a0a0a #dfdfdf !important;
		box-shadow: rgb(0 0 0 / 35%) 4px 4px 10px 0, rgb(254 254 254) 1px 1px 0 1px inset,
			rgb(132 133 132) -1px -1px 0 1px inset !important;
		box-sizing: border-box !important;
		display: inline-block;
		width: 100%;
		height: 100%;
		background: #c6c6c6 !important;
		color: #0a0a0a !important;
	}
	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		align-items: center;
	}

	.connectBtn {
		position: fixed;
		right: 10px;
		top: 10px;
	}

	.container {
		display: grid;
		padding: 0;
	}

	.content,
	.overlay {
		grid-area: 1 / 1;
	}
	canvas {
		padding: 0;
	}

	.start {
		background: silver;

		height: 36px;
		font-size: 16px;
		text-align: center;

		font-weight: bold;
		cursor: pointer;
		box-shadow: inset -1px -1px #0a0a0a, inset 1px 1px #fff, inset -2px -2px grey,
			inset 2px 2px #dfdfdf;
	}

	.start:hover {
		border-width: -1px;
		background: rgb(185, 185, 185);
	}
	.start:active {
		box-shadow: inset -1px -1px #fff, inset 1px 1px #0a0a0a, inset -2px -2px #dfdfdf,
			inset 2px 2px grey;
		padding-bottom: 1px;
		padding-right: 1px;
	}
</style>
