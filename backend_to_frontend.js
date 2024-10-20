const userAddress = "0x1985EA6E9c68E1C272d8209f3B478AC2Fdb25c87"
let chain = "BASE"

let request = {
    user_address: userAddress,
    chain: chain,
}

const LOCALHOST = "127.0.0.1"
const port = 8080

var method = "erc20"

async function makeRequest() {
    try {
        const response = await fetch(`http://${LOCALHOST}:${port}/${method}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            return response.json().then(errData => {
                // Log the error data for debugging
                console.error('Error Data:', errData);

                // Throw an error with a more descriptive message
                throw new Error(errData.message || 'Unknown error occurred');
            });
        }

        const data = await response.json();  // Parse response as JSON

        // Access the nested fields
        const userDetails = data.user_details;
        const details = userDetails.details;
        const portfolioValue = userDetails.portfolio_value;
        const err = data.err;

        if (err) {
            console.error('Error:', err);
        } else {
            console.log('Details:', details);
            console.log('Portfolio Value:', portfolioValue);
        }
        
        // Example to get details of one token:
        token_name = details[0].token_name
        token_decimals = details[0].token_decimals
        token_balance = parseInt(details[0].token_balance, 16) / token_decimals
        token_chain = details[0].chain
        token_price = details[0].token_price        // can be null 

        // console.log(token_balance, token_price, token_name, token_chain, token_decimals)

    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

// Call the async function
makeRequest();

method = "erc721"

request = {
    user_address: "0x439c36f21d961Dc81Bfb39331845FbDC8C9E8be8",
    chain: "BASE",
}

async function makeNftRequest() {
    try {
        const response = await fetch(`http://${LOCALHOST}:${port}/${method}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            return response.json().then(errData => {
                // Log the error data for debugging
                console.error('Error Data:', errData);

                // Throw an error with a more descriptive message
                throw new Error(errData.message || 'Unknown error occurred');
            });
        }

        const data = await response.json();  // Parse response as JSON
        console.log("Response data:", data);  // Log the response data
        console.log("NFT urls:", data.nft_details[0].media_urls);  // Log the response data

        // Example NFT Data
        nft1 = data.nft_details[0]
        nft1_name = nft1.name
        nft1_symbol = nft1.symbol
        nft1_description = nft1.description
        nft1_mediaUrls = nft1.media_urls
        nft1_collectionName = nft1.collection_name
        nft1_floorPriceEth = nft1.floor_price_eth
        nft1_lastTradedPriceUsd = nft1.last_traded_price_usd

        // console.log(nft1_name,nft1_symbol,nft1_description,nft1_mediaUrls,nft1_collectionName,nft1_floorPriceEth,nft1_lastTradedPriceUsd)
    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

makeNftRequest();

method = "transaction-history"

chain = "BASE"
request = {
    user_address: userAddress,
    chain: chain,
}

async function makeTransactionRequest() {
    try {
        const response = await fetch(`http://${LOCALHOST}:${port}/${method}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            return response.json().then(errData => {
                // Log the error data for debugging
                console.error('Error Data:', errData);

                // Throw an error with a more descriptive message
                throw new Error(errData.message || 'Unknown error occurred');
            });
        }

        const data = await response.json();  // Parse response as JSON
        console.log("Response data:", data);  // Log the response data

        // Example Transaction Data
        complete_array_of_transaction_details = data.transaction_details
        
        transaction1 = data.transaction_details[0]
        transaction1_from = transaction1.from
        transaction1_to = transaction1.to
        transaction1_value = parseInt(transaction1.value, 16)
        transaction1_blockNumber = parseInt(transaction1.block_number, 16)
        transaction1_EtherScanUrl = getTransactionUrl(transaction1.txn_hash, chain)

        // console.log(transaction1_from,transaction1_to,transaction1_value,transaction1_blockNumber,transaction1_EtherScanUrl)
    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

makeTransactionRequest();

function getTransactionUrl(txHash, network) {
    switch (network) {
        case "ETHEREUM":
            return `https://etherscan.io/tx/${txHash}`;
        case "BASE":
            return `https://basescan.org/tx/${txHash}`;
        default:
            return `Unknown network: ${network}`;
    }
}