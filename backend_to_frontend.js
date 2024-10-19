const userAddress = "0x9d9259cD27020E84E3Bb820C75498D468CaC98B5"
const chain = "BASE"

const request = {
    user_address: userAddress,
    chain: chain,
}

const LOCALHOST = "127.0.0.1"
const port = 8080

var method = "erc20"

console.log(`http://${LOCALHOST}:${port}/${method}`);
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
        console.log("Response data:", data);  // Log the response data

    } catch (error) {
        console.error('There was a problem with the fetch operation:', error);
    }
}

// Call the async function
makeRequest();