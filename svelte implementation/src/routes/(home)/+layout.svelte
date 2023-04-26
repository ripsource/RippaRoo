<script>
import { slide, fly } from 'svelte/transition';
import { RipCurAddress, chosenCollection, ripCollesh, chosenOne, wallet, newRip, ripEdish, ripTotalCollection, announcements } from '../../stores';
import {goto} from '$app/navigation';

function socialsaway() {
  
   announcements.set('none')
}

let results = ''
let resultsCollection = ''


        
        let rippy = [];
        let traits = [];
        let rippyLoad = [];
     
        let rippyEditionNumber = ''
        $: console.log(rippyEditionNumber)


let rippyAddress


        
onMount(async () => {
           
    
    const res = await fetch(`https://raw.githubusercontent.com/ripsource/data/main/RippyOwners%20(1).json`);
    rippy = await res.json();
   rippy = rippy.NFT
   traits = rippy
   rippyLoad = rippy
   ripTotalCollection.set(rippyLoad)
 
   let queryString = window.location.search;
let urlParams = new URLSearchParams(queryString);
let numString = urlParams.get('number')
console.log(numString + 'jdsjdlkjsdlkjfslkdfjslkfdj')

let numString2 = urlParams.get('collection')
console.log(numString2 + 'jdsjdlkjsdlkjfslkdfjslkfdj')



resultsCollection = $ripTotalCollection.filter(x => x.fullWallet === numString2);
    chosenCollection.set(resultsCollection)


results = $ripTotalCollection.filter(x => x.EditionNumber === '#' + numString);
    chosenOne.set(results[0])

    
});



    





function setRippy () {
   
    ripEdish.set(rippyEditionNumber)
    console.log('helloworld')
    rippyEditionNumber = rippyEditionNumber
    results = $ripTotalCollection.filter(x => x.EditionNumber === '#' + rippyEditionNumber);
    chosenOne.set(results[0])
console.log(results)
}

let ripCur = ''

function setRippyAddress () {
   
   ripCollesh.set(rippyAddress)
   console.log('helloworldCollection')
   rippyAddress = rippyAddress
ripCur = rippyAddress.slice(57,65)
console.log(ripCur)
RipCurAddress.set(ripCur)
   
   resultsCollection = $ripTotalCollection.filter(x => x.fullWallet === rippyAddress);
   chosenCollection.set(resultsCollection)
console.log(resultsCollection)


}





    import { onMount } from 'svelte';
        
        let xrd = [];
        let todayPrice = []
         onMount(async () => {
           
    
             const res = await fetch(`https://api.coingecko.com/api/v3/simple/price?ids=radix&vs_currencies=usd`);
             xrd = await res.json();
             console.log(xrd)
            todayPrice = xrd.radix.usd.toFixed(2);
            
    
            
         });
         let show = 'none'


function anywhere () {
    if (show === 'block') {
        show = 'none'
    }
}


function open () {
	if (show === 'none') {
    show = 'block' }
   
    else {
    show = 'none' 
   
    }
}
console.log($wallet)

function collection() {
    goto("/collection")
   
}
function logout() {
    goto("/")
   
}

function myFiles() {
    goto("/files")
   
}

function system() {
    goto("/system")
   
}

function socials() {
    goto("/socials")
   
}

function media() {
    goto("/media")
   
}


function ripPaint () {
    goto("/paint")
}

let extendmenu = "none"
function displayProp () {
    if (extendmenu === "none") {
        extendmenu = "block"
    } else {
        extendmenu = "none"
    }

}

function displayProp2 () {
    if (extendmenu === "block") {
        extendmenu = "none"
    }
}

function chart2 () {
    if (extendmenu2 === "block") {
        extendmenu2 = "none"
    }
}


let extendmenu2 = "none"
function chart () {
    if (extendmenu2 === "none") {
        extendmenu2 = "block"
    } else {
        extendmenu2 = "none"
    }

}

function rippyroo () {
    goto("/rippyroo")
}
function statistics () {
    goto("/stats")
}

function rarities () {
    goto("/rarity")
}

let wallpaper = "classic"

function rip0 (){
 wallpaper = "classic"
}

function rip1 (){
 wallpaper = "averip"
}

function rip2 (){
wallpaper = "ripic"
}
function rip3 (){
wallpaper = "inripior"
}
function rip4 (){
wallpaper = "ripmentary"
}
function rip5 (){
wallpaper = "ultrip"
}

import {clickOutside} from './clickOutside';
	
	function handleClickOutside(event) {
		show = "none"
        extendmenu = "none"
	}


 
    </script>



   



<div class="{wallpaper}">  
   <div use:clickOutside on:click_outside={handleClickOutside}> 
{#if show === "block"}
<div in:slide="{{ y: 300, duration: 500 }}" class="programs" style="display: {show}; position: fixed; bottom: 36px; width: 300px; z-index: 100;">
	<div style="height: 5px; background: #060084;"></div>
	<div style="height: 50px; word-break: break-all; font-size: 24px; color: white; font-weight: bold; background: #060084; padding: 0 4px 0 4px; position: relative;"><img alt="user" style="width: 39px; position: absolute; margin-left: 5px;" src="computer.png"><span style="margin-left: 55px; line-height: 30px; line-height: 50px;">rdx...{$wallet}</span></div>
	<button on:click={collection} on:click={open} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='collection' style="width: 30px; position: absolute;" src="Collection.png"><span style="margin-left: 35px; line-height: 30px;">Collection Viewer</span></button>
	<button on:click={myFiles} on:click={open} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='system info' style="width: 30px; position: absolute;" src="usercollection.png"><span style="margin-left: 35px; line-height: 30px;">My Files</span></button>
	<button on:click={system} on:click={open} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='system info' style="width: 30px; position: absolute;" src="Instructions.png"><span style="margin-left: 35px; line-height: 30px;">System Information</span></button>
	<button on:click={chart} on:click={displayProp2} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='system info' style="width: 30px; position: absolute;" src="RarityChart.png"><span style="margin-left: 35px; line-height: 30px;">Data</span></button>
	
    <button on:click={socials} on:click={open} on:click={socialsaway} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='socials' style="width: 30px; position: absolute;" src="Telegram.png"><span style="margin-left: 35px; line-height: 30px;">Communications</span></button>
	<button on:click={media} on:click={open} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='videos' style="width: 30px; position: absolute;" src="Rippy Media.png"><span style="margin-left: 35px; line-height: 30px;">Rippy Clips</span></button>
	<hr>
   
    <form on:click={setRippy} on:keyup={setRippy} action="./rippy">
        <input bind:value={rippyEditionNumber} class="address" style="width: 100%; height: 25px;" name="number" id="formEntry" placeholder="0001 - 2222" type="text"  />
       
    </form>
    <form on:click={setRippyAddress} on:keyup={setRippyAddress} action="./rippy-collection" style="margin-bottom: 2px;">
        <input bind:value={rippyAddress} class="address" style="width: 100%; height: 25px;" name="collection" id="formEntry" placeholder="rdxqsp1..." type="text"  />
       
    </form>


    <button on:click={displayProp} on:click={chart2} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='videos' style="width: 30px; position: absolute;" src="displayProp.png"><span style="margin-left: 35px; line-height: 30px;">Personalise</span></button>
      
  
	
	<button on:click={logout} class="programsList" style="position: relative; padding-left: 5px; font-weight: bold;"><img alt='videos' style="width: 30px; position: absolute;" src="keys-4.png"><span style="margin-left: 35px; line-height: 30px;">Log Out</span></button>

</div>

{#if extendmenu === "block"}
<div in:fly="{{ x: -300, duration: 500 }}" class="personaliseList" style="position: fixed; display: {extendmenu};">
    <button on:click={ripPaint} on:click={open} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: #fffff" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Creator</span></button>
    <button on:click={rip0} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: #008080" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Simply Rip</span></button>
    <button on:click={rip4} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: yellow;" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Ripmentary</span></button>
	<button on:click={rip3} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: green;" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Inripior</span></button>
	
    <button on:click={rip1} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: red;" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Averip</span></button>
	<button on:click={rip2} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: blue;" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Ripic</span></button>
{#if $newRip === "block"}
<button on:click={rip5} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><i style="color: orange;" class="fa-solid fa-circle"></i><span style="margin-left: 5px; line-height: 30px;">Ultrip</span></button>
{/if}




</div>
{/if}
{#if extendmenu2 === "block"}
<div in:fly="{{ x: -300, duration: 500 }}" class="statList" style="position: fixed; display: {extendmenu2};">
<button on:click={rarities} on:click={open} on:click={chart} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><span style="margin-left: 5px; line-height: 30px;">Rarities</span></button>
<button on:click={statistics} on:click={open} on:click={chart} class="programsList2" style="position: relative; padding-left: 5px; font-weight: bold;"><span style="margin-left: 5px; line-height: 30px;">Owner Stats</span></button>
</div>
{/if}
{/if}
</div>
<div class="viewport">
<slot></slot>


    
    
</div>  
</div>
    <style>

hr {
    margin-bottom: 0;
}

input {
    margin: 0 !important;
}

.personaliseList {
    bottom: 84px; width: 150px; left: 300px;
}

.statList {
    bottom: 280px; width: 150px; left: 300px;
}

.address {
    background: rgb(227, 227, 227);
}
@media only screen and (max-width: 600px) {
    .personaliseList {
    bottom: 511px !important; width: 150px; left: 150px !important; z-index: 100; outline: 1px solid white; background: white;
}

}

@media only screen and (max-height: 695px) {
    .personaliseList {
    bottom: 83px !important; width: 150px; left: 180px !important; z-index: 100; outline: 1px solid white; background: white;
}

}

@media only screen and (max-width: 600px) {
    .statList {
    bottom: 511px !important; width: 150px; left: 150px !important; z-index: 100; outline: 1px solid white; background: white;
}

}

@media only screen and (max-height: 700px) {
    .statList {
    bottom: 283px !important; width: 150px; left: 180px !important; z-index: 100; outline: 1px solid white; background: white;
}

}



.viewport {
    height: 100vh;
}
.programsList {
	
   
    width: 100%;
    text-align: left;
	background: #c6c6c6;
    color: black;
    padding: 0 0 4px;
    border-radius: 1px;
	height: 40px;
	margin: 2px 0 1px 0;
}

.programsList2 {
	
   
    width: 100%;
    text-align: left;
	background: #c6c6c6;
    color: black;
    padding: 0 0 4px;
    border-radius: 1px;
	height: 40px;

}

.programsList:hover {

outline: 1px solid rgb(156, 156, 156) 
}

.programs {
	padding: 2px;
    font-size: 1rem;
    border-style: solid;
    border-width: 2px;
    border-color: #848584 #fefefe #fefefe #848584;
    line-height: 1.5;
    background: #c6c6c6;
	width: 100%;
}

.classic {
    background: #008080;
}




.averip {
    background-image: url(/rarechestWallpaper.webp);
    background-size: cover;
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
    background-color: #008080;
}
.ripic {
    background-color: #008080;
    background-image: url(/trippyrippy3.webp);
    background-size: cover;
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
}
.inripior {
    background-color: #008080;
    background-image: url(/Sunflowers.webp);
    background-size: cover;
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
}
.ripmentary {
    background-color: #008080;
    background-image: url(/PinkRepeat.webp);
    background-size: cover;
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
}
.ultrip {
    background-color: #008080;
    background-image: url(/cleanstreak.webp);
    background-size: cover;
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
}

button {
    margin: 0;
}


        .bar {
            position: fixed;
            bottom: 0;
            grid-row-start: 2;
  grid-column: 1 / span 2;
  background: #c6c6c6;
        }

        .items {
            display: grid;
            grid-template-columns: 210px 1fr 98px;
            width: 100vw;
        }
    
    
        .start {
            background: rgb(215, 215, 215);
      
        height: 36px;
        font-size: 16px;
        text-align: center;
       
        font-weight: bold;
        cursor: pointer;
    width: 210px;
        }
    
        .start:active {
            box-shadow: inset -1px -1px #ffffff, inset 1px 1px #0a0a0a, inset -2px -2px #dfdfdf, inset 2px 2px #808080;
            padding-bottom: 1px;
            padding-right: 1px;
        }
    
        .container {
      display: grid;
      padding: 0;
    }
    
    .content, .overlay {
      grid-area: 1 / 1;
    }
    

    :global(body *::-webkit-scrollbar) {
  width: 16px;
  height: 16px;
  background: none;
}

:global(body *::-webkit-scrollbar-thumb, body *::-webkit-scrollbar-button) {
  width: 16px;
  height: 16px;
  background: silver;
  box-shadow: inset 1px 1px #dfdfdf, inset -1px -1px gray;
  border: 1px solid;
  border-color: silver #000 #000 silver;
}

:global(body *::-webkit-scrollbar-track) {
  image-rendering: optimizeSpeed;
  image-rendering: pixelated;
  image-rendering: optimize-contrast;
  background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgLTAuNSAyIDIiIHNoYXBlLXJlbmRlcmluZz0iY3Jpc3BFZGdlcyI+CjxtZXRhZGF0YT5NYWRlIHdpdGggUGl4ZWxzIHRvIFN2ZyBodHRwczovL2NvZGVwZW4uaW8vc2hzaGF3L3Blbi9YYnh2Tmo8L21ldGFkYXRhPgo8cGF0aCBzdHJva2U9IiNjMGMwYzAiIGQ9Ik0wIDBoMU0xIDFoMSIgLz4KPC9zdmc+");
  background-position: 0 0;
  background-repeat: repeat;
  background-size: 2px;
}

:global(body *::-webkit-scrollbar-button) {
  background-repeat: no-repeat;
  background-size: 16px;
}

:global(body *::-webkit-scrollbar-button:single-button:vertical:decrement) {
  background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgLTAuNSAxNiAxNiIgc2hhcGUtcmVuZGVyaW5nPSJjcmlzcEVkZ2VzIj4KPG1ldGFkYXRhPk1hZGUgd2l0aCBQaXhlbHMgdG8gU3ZnIGh0dHBzOi8vY29kZXBlbi5pby9zaHNoYXcvcGVuL1hieHZOajwvbWV0YWRhdGE+CjxwYXRoIHN0cm9rZT0iIzAwMDAwMCIgZD0iTTcgNWgxTTYgNmgzTTUgN2g1TTQgOGg3IiAvPgo8L3N2Zz4=");
}

:global(body *::-webkit-scrollbar-button:single-button:vertical:increment) {
  background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgLTAuNSAxNiAxNiIgc2hhcGUtcmVuZGVyaW5nPSJjcmlzcEVkZ2VzIj4KPG1ldGFkYXRhPk1hZGUgd2l0aCBQaXhlbHMgdG8gU3ZnIGh0dHBzOi8vY29kZXBlbi5pby9zaHNoYXcvcGVuL1hieHZOajwvbWV0YWRhdGE+CjxwYXRoIHN0cm9rZT0iIzAwMDAwMCIgZD0iTTQgNWg3TTUgNmg1TTYgN2gzTTcgOGgxIiAvPgo8L3N2Zz4=");
}

:global(body *::-webkit-scrollbar-button:single-button:horizontal:decrement) {
  background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgLTAuNSAxNiAxNiIgc2hhcGUtcmVuZGVyaW5nPSJjcmlzcEVkZ2VzIj4KPG1ldGFkYXRhPk1hZGUgd2l0aCBQaXhlbHMgdG8gU3ZnIGh0dHBzOi8vY29kZXBlbi5pby9zaHNoYXcvcGVuL1hieHZOajwvbWV0YWRhdGE+CjxwYXRoIHN0cm9rZT0iIzAwMDAwMCIgZD0iTTggM2gxTTcgNGgyTTYgNWgzTTUgNmg0TTYgN2gzTTcgOGgyTTggOWgxIiAvPgo8L3N2Zz4=");
}

:global(body *::-webkit-scrollbar-button:single-button:horizontal:increment) {
  background-image: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgLTAuNSAxNiAxNiIgc2hhcGUtcmVuZGVyaW5nPSJjcmlzcEVkZ2VzIj4KPG1ldGFkYXRhPk1hZGUgd2l0aCBQaXhlbHMgdG8gU3ZnIGh0dHBzOi8vY29kZXBlbi5pby9zaHNoYXcvcGVuL1hieHZOajwvbWV0YWRhdGE+CjxwYXRoIHN0cm9rZT0iIzAwMDAwMCIgZD0iTTYgM2gxTTYgNGgyTTYgNWgzTTYgNmg0TTYgN2gzTTYgOGgyTTYgOWgxIiAvPgo8L3N2Zz4=");
}

:global(body *::-webkit-scrollbar-corner) {
  background: silver;
}




    </style>
    