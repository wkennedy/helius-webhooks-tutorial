Helius-webhooks-tutorial

curl --header "Content-Type: application/json" \
--request POST \
--data '[{"blockTime":1673445241,"indexWithinBlock":2557,"meta":{"err":null,"fee":10000,"innerInstructions":[null],"loadedAddresses":[null],"logMessages":[null],"postBalances":[null],"postTokenBalances":[null],"preBalances":[null],"preTokenBalances":[null],"rewards":[]},"slot":171942732,"transaction":{"message":[null],"signatures":[null]}}]' \
http://localhost:3000/webhook