use scrypto::prelude::*;

    
    //mutable NFT score card
    
    
    #[derive(ScryptoSbor, NonFungibleData)]
    pub struct MyNFTData {
        pub name: String,
        pub key_image_url: String,
        pub persona: String,
        pub user_account: String,
        #[mutable]
        pub score_total: Decimal,
    }

    
    #[blueprint]
    mod rippy_machine {
        struct RippyMachine {
            ripper_tokens: Vault,
            collected_xrd: Vault,
            nft_resource: ResourceAddress,
            admin_badge: Vault,
            
           
           
        }
    
        impl RippyMachine {
          
            pub fn instantiate_rippy_machine() -> (ComponentAddress, Bucket) {
                // create new score tokens
                let bucket_of_rips = ResourceBuilder::new_fungible()
                    .metadata("name", "Rippy Roo")
                    .metadata("symbol", "ROO")
                    .metadata("description", "Score Points on Rippy Roo")
                    .metadata("tags", "gaming")
                    .metadata("icon_url", "https://rippyclip.com/RippyRoo.png")
                    .metadata("info_url", "https://rippyclip.com")
                    .mint_initial_supply(999999999);
    
                // create admin badge
                    let admin_badge: Bucket = ResourceBuilder::new_fungible()
                    .metadata("name", "admin_badge")
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);
                
                    let roo_badge: Bucket = ResourceBuilder::new_fungible()
                    .metadata("name", "admin badge")
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);



                // create score card nft
                    let nft_resource: ResourceAddress = ResourceBuilder::new_uuid_non_fungible::<MyNFTData>()
                    .metadata("name", "Rippy Roo Score Card")
                    .metadata("description", "A score card that tracks your highscore in Rippy Roo")
                    .metadata("key_image_url", "https://rippyclip.com/CoverIMG.png")
                    .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                    .updateable_non_fungible_data(
                        rule!(require(admin_badge.resource_address())), LOCKED)
                    .create_with_no_initial_supply();
    
    
            
                let component = Self {
                    ripper_tokens: Vault::with_bucket(bucket_of_rips),
                    collected_xrd: Vault::new(RADIX_TOKEN),
                    nft_resource,
                    admin_badge: Vault::with_bucket(admin_badge),
                   
                   
                }
                .instantiate();

                let access_rules = AccessRulesConfig::new()
                .method(
                    "withdraw_roo",
                    rule!(require(roo_badge.resource_address())),
                    LOCKED,
                )
                .default(AccessRule::AllowAll, AccessRule::DenyAll);
            (
                component.globalize_with_access_rules(access_rules),
                roo_badge,
            )
            }
    
         
    // just an example method allowing users to send back ROO tokens
            pub fn donate_roo(&mut self, roobucket: Bucket) {
                self.ripper_tokens.put(roobucket);
            }
    // just an example method enabling admin to withdraw all ROO tokens
            pub fn withdraw_roo(&mut self) {
                self.ripper_tokens.take_all();
            }
            
    
    // This method mints a new score card, with a starting score of 0 in its metadata, the Persona used when they connected to the dApp and their wallet address.
            pub fn mint_score_card(&mut self, persona_input: String, account_input: String) -> Bucket {
                let nft: Bucket = self.admin_badge.authorize(|| {
                  let resource_manager = borrow_resource_manager!(self.nft_resource);
                  resource_manager.mint_uuid_non_fungible(MyNFTData{name: String::from("High Score"), key_image_url: String::from("https://rippyclip.com/CoverIMG.png"), persona: persona_input, user_account: account_input, score_total: dec!(0)},
                  )
                });
    
                return nft
            }
    
    

    //This method allows a user to update their NFT with a their new high score, it also pays out ROO tokens equivalant to the score (i.e. if they have a highscore of 5 and therefore own 5 ROO tokens, then get a new highscore of 6, they will only receive back 1 new ROO token). 
    //A token is issued because its not possble to see metadata in the radix beta wallet and also it's fun to receive tokens
    //However, the token may not be entirely redundant as it could be used to buy in-game upgrades, etc. if develeoped further 
            pub fn claim_high_score(&mut self, auth: Proof, final_score: Decimal) -> Bucket {
                let auth = auth.validate_proof(self.nft_resource).expect("Invalid Score Card Passed"); 
            
                let nft: NonFungible<MyNFTData> = auth.non_fungible();
                let mut nft_data: MyNFTData = nft.data();
                //check they actually got a higher score than whatis already recorded in the NFT score card
                let condition = nft_data.score_total >= final_score;
                assert!(!condition, "You haven't got a higher score"); 
            
                //only payout the difference between previous score and new high score
                let payout = final_score - nft_data.score_total;
            
            // update FNT metadata with the new high score
                nft_data.score_total = final_score;
                self.admin_badge.authorize(|| {  
                    borrow_resource_manager!(self.nft_resource).update_non_fungible_data(
                        &nft.local_id(), 
                        "score_total",
                        final_score)
                });
                
                let reward = self.ripper_tokens.take(payout);
                return reward
            }
    
    
    
        }
    }
    
