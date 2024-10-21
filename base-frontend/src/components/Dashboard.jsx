import React from 'react';
import { Bitcoin } from 'lucide-react';
import GradientLineChart from './Chart';
import FloatingOverlay from './Dash';

const Dashboard = ({ erc20Data, erc721Data, transactionData, loading }) => {
  if (loading) {
    return (
      <div className="flex justify-center items-center h-full">
        <p>Loading...</p>
      </div>
    );
  }

  if (!erc20Data || !erc721Data) {
    return <p>No data available.</p>;
  }

  const erc20Details = erc20Data.user_details.details;
  const nftDetails = erc721Data.nft_details;
  const erc20PortfolioValue = Number(erc20Data.user_details.portfolio_value) || 0;
  const nftPortfolioValue = Number(erc721Data.nft_portfolio_value) || Number(erc20PortfolioValue*0.12);
  const netWorth = erc20PortfolioValue + nftPortfolioValue;

  const tokens = erc20Details.map((tokenDetail) => {
    const tokenName = tokenDetail.token_name;
    const tokenSymbol = tokenDetail.token_symbol;
    const tokenBalance =
      parseInt(tokenDetail.token_balance, 16) /
      Math.pow(10, tokenDetail.token_decimals);
    const tokenPrice = tokenDetail.token_price || 0;
    const tokenValue = tokenBalance * tokenPrice;
    const percentage = ((tokenValue / netWorth) * 100).toFixed(2) + '%';

    return {
      name: tokenSymbol,
      percentage: percentage,
      priceChange: '+$0 (0%)', // Placeholder for price change
      price: `$${tokenPrice.toFixed(2)}`,
      balance: `${tokenBalance.toFixed(4)} ${tokenSymbol}`,
    };
  });

  return (
    <div>
      <main className="bg-[#131313] p-2 rounded-3xl rounded-tl-none pt-6 mt-[60px]">
        <div className="bg-[#1E1E1E] p-4 pl-10 ml-[10px] mr-[15px] pr-0 mb-8 rounded-lg">
          <div className="grid grid-cols-3 gap-3">
            <div className="bg-[#232323] ml-[-18px] rounded-lg mr-[25px] p-4 py-2">
              <h3 className="text-white font-inter font-semibold mb-2">
                Net Worth
              </h3>
              <div className="flex flex-col">
                <span className="text-4xl font-inter font-bold mb-2">
                  ${netWorth.toFixed(2)}
                </span>
                <div className="text-green-500 font-inter font-semibold text-sm">
                  +2.27% (${(netWorth.toFixed(2)*0.0227).toFixed(2)})
                </div>
              </div>
            </div>
            <div className="bg-[#232323] ml-[-18px] rounded-lg mr-[25px] p-4 py-2">
              <h3 className="text-white font-inter font-semibold mb-2">
                Token Worth
              </h3>
              <div className="flex flex-col">
                <span className="text-4xl font-inter font-bold mb-2">
                  ${erc20PortfolioValue.toFixed(2)}
                </span>
                <div className="text-green-500 font-inter font-semibold text-sm">
                  +1.67% (${(erc20PortfolioValue.toFixed(2)*0.0167).toFixed(2)})
                </div>
              </div>
            </div>
            <div className="bg-[#232323] ml-[-18px] rounded-lg mr-[25px] p-4 py-2">
              <h3 className="text-white font-inter font-semibold mb-2">
                NFTs Worth
              </h3>
              <div className="flex flex-col">
                <span className="text-4xl font-inter font-bold mb-2">
                  ${nftPortfolioValue.toFixed(2)}
                </span>
                <div className="text-green-500 font-inter font-semibold text-sm">
                  +0.69% (${(nftPortfolioValue.toFixed(2)*0.0069).toFixed(2)})
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-transparent pt-8 p-4 ml-[-6px] rounded-lg mt-[-52px]">
          <div className="grid grid-cols-3 gap-4">
                <div className="col-span-2 bg-[#1E1E1E] rounded-lg pt-6 pb-0 pl-8 pr-0">
                    <div className="flex justify-between items-center mb-4">
                    <h3 className="font-bold">Tokens Worth</h3>
                    <div className="flex space-x-1 pr-9">
                        <button className="bg-transparent px-2 py-1 rounded-md text-sm font-bold">
                        24H
                        </button>
                        <button className="bg-white px-3 h-[20px] mt-1 rounded-md text-black text-sm font-bold">
                        7D
                        </button>
                        {['30D', '6M'].map((period) => (
                        <button key={period} className="bg-transparent px-2 py-1 rounded-md text-sm font-bold">
                            {period}
                        </button>
                        ))}
                    </div>
                    </div>
                    <GradientLineChart />
                </div>
            <div className="bg-[#1E1E1E] rounded-lg p-4">
              <h3 className="font-bold mb-4">NFTS</h3>
              <div className="flex justify-between mb-4">
                <div className="bg-[#373737] p-3 w-40 rounded-md">
                  <p className="text-gray-400">Collections</p>
                  <p className="text-xl font-bold">{nftDetails.length}</p>
                </div>
                <div className="bg-[#373737] p-3 w-28 rounded-md">
                  <p className="text-gray-400">NFTS</p>
                  <p className="text-xl font-bold">
                    {nftDetails.reduce(
                      (acc, nft) => acc + (nft.quantity || 1),
                      0
                    )}
                  </p>
                </div>
              </div>
              <button className="w-full mt-32 bg-[#DA0046] text-white font-bold rounded-md py-2">
                OPEN NFTS
              </button>
            </div>
          </div>
        </div>

        <div className="bg-transparent p-4 ml-[-6px] rounded-lg mt-[-20px]">
          <div className="bg-[#1E1E1E] rounded-lg p-4">
            {tokens.map((token, index) => (
              <div
                key={index}
                className="flex items-center justify-between py-4 px-3 bg-[#232323] rounded-lg mb-2"
              >
                <div className="flex items-center">
                  <div className="w-8 h-8 mr-3 bg-gray-600 rounded-full flex items-center justify-center">
                    <Bitcoin size={20} />
                  </div>
                  <div>
                    <div className="font-semibold">{token.name}</div>
                    <div className="text-sm text-gray-400">Token</div>
                  </div>
                </div>
                <div className="text-right">
                  <div>{token.percentage}</div>
                  <div className="text-green-500 text-sm">
                    {token.priceChange}
                  </div>
                </div>
                <div className="text-right">
                  <div>{token.price}</div>
                  <div className="text-sm text-gray-400">{token.balance}</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </main>
      <FloatingOverlay />
    </div>
  );
};

export default Dashboard;
