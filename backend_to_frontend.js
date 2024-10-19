const userAddress = "0x1985EA6E9c68E1C272d8209f3B478AC2Fdb25c87"
const chain = "BASE"

const request = {
    user_address: userAddress,
    chain: chain,
}

const LOCALHOST = "127.0.0.1"
const port = 8080

var method = "erc20"

// console.log(`http://${LOCALHOST}:${port}/${method}`);
console.log(JSON.stringify(request));
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
        // console.log("Response data:", data);  // Log the response data

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

            // Example: Log the first detail
            if (details.length > 0) {
                console.log('First Token:', details[0].token_name);
                console.log('First Token Balance:', details[0].token_balance);
            }
        }

    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

// Call the async function
makeRequest();

// method = "erc721"

// async function makeNftRequest() {
//     try {
//         const response = await fetch(`http://${LOCALHOST}:${port}/${method}`, {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/json',
//             },
//             body: JSON.stringify(request),
//         });

//         if (!response.ok) {
//             return response.json().then(errData => {
//                 // Log the error data for debugging
//                 console.error('Error Data:', errData);

//                 // Throw an error with a more descriptive message
//                 throw new Error(errData.message || 'Unknown error occurred');
//             });
//         }

//         const data = await response.json();  // Parse response as JSON
//         console.log("Response data:", data);  // Log the response data

//     } catch (error) {
//         console.error('There was a problem with the fetch operation:', error);
//     }
// }

// makeNftRequest();

method = "transaction-history"

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

    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

makeTransactionRequest();

function getTransactionUrl(txHash, network) {
    switch (network) {
        case "ethereum":
            return `https://etherscan.io/tx/${txHash}`;
        case "base":
            return `https://basescan.org/tx/${txHash}`;
        default:
            return `Unknown network: ${network}`;
    }
}