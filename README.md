### Project description

The app allows users to purchase cases (boxes) with random rewards (tokens or NFTs). Users can buy any amount of boxes per tx. The app supports native or IBC tokens as payment currency. Users can open bought boxes (1 per tx) or give to other user (any amount). Every box contains fixed amount of tokens with drop chance specified according to distribution config. Here is example of such config
| Box Rewards, $STARS | Drop Chance, % |
| :-: | :-: |
| 0 | 46 |
| 50 | 40 |
| 250 | 8 |
| 500 | 4 |
| 1000 | 2 |

If the app rewards pool contains NFTs there is 50 % chance to get NFT instead of tokens. Each NFT has a price specified by app owner and it makes possible to determine what NFT must be given to user. For example if a user opened 500 $STARS box (4 % drop chance) he can get 500 $STARS with 50 % chance or NFT with 500 $STARS price (written in the contract) with same 50 % chance.

If the app has enough liquidity it will send tokens to user opened a box immediately. Otherwise rewards will be accumulated and available to claim later. The app owner must deposit initial liquidity in tokens to ensure rewards distribution timely. NFT rewards must be deposited in small amounts to keep specified price close to market prices (NFT prices can be updated but they always must be equal to one of the box rewards value).

The platform contract works as box factory. It can be instantiated from treasury contract that provides common balance for all platforms. By default platform will be included in platform list but it's possible to move it removed platform list. Then users will not be able to buy new boxes on the platfrom but still be able to open, send and claim.
