import React from 'react';
import { Bitcoin, Clock, CheckCircle, XCircle } from 'lucide-react';
import FloatingOverlay from './Txn';

const TransactionList = () => {
  const transactions = [
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0x8cfb1552...f081aaa29e',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0x647e5af3...bd4be1e411',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xb6dbacfb...cede5e95f6',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xe330bc08...77577a73a2',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0x38242d07...d70a09a0cf',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xd7b749b7...3bf32318d9',
        from: '0x53c...c03Ec',
        to: '0x786...95609'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0x1b18adc2...284638ff17',
        from: '0x53c...c03Ec',
        to: '0x000...eB395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 1120.0,
        id: '0xa3585d15...d693c1659d',
        from: '0x53c...c03Ec',
        to: '0x01E...5d7cB'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xe219eb4c...4ad37924e3',
        from: '0x53c...c03Ec',
        to: '0xbe3...F649F'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 58.8,
        id: '0xf1e3a3b2...30b4333815',
        from: '0x53c...c03Ec',
        to: '0xbe3...F649F'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xd92d16fc...fde56bb9d8',
        from: '0x53c...c03ec',
        to: '0x000...eb395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0xc6e53989...0a14d59b64',
        from: '0x53c...c03ec',
        to: '0x000...eb395'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 420.0,
        id: '0xdac00017...c78f75d5e9',
        from: '0x53c...c03ec',
        to: '0x2e8...b2d03'
      },
      {
        icon: 'primary',
        status: 'Success',
        amount: 0.0,
        id: '0x9211853a...de4c4461e2',
        from: '0x53c...c03ec',
        to: '0x2e8...b2d03'
      }
  ];

  const getIcon = (type) => {
    switch (type) {
      case 'primary':
        return <Bitcoin className="w-6 h-6 text-blue-400" />;
      case 'success':
        return <Bitcoin className="w-6 h-6 text-green-500" />;
      case 'warning':
        return <Bitcoin className="w-6 h-6 text-orange-500" />;
      case 'error':
        return <Bitcoin className="w-6 h-6 text-red-500" />;
      default:
        return <Bitcoin className="w-6 h-6 text-gray-500" />;
    }
  };

  const getStatusIcon = (status) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'pending':
        return <Clock className="w-5 h-5 text-orange-500" />;
      case 'failed':
        return <XCircle className="w-5 h-5 text-red-500" />;
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-[#131313] p-6 mt-[60px] rounded-2xl rounded-tl-none">
      <div className="max-w-6xl mx-auto bg-[#1E1E1E] rounded-xl shadow-lg overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="text-gray-400 text-sm">
                <th className="py-4 px-6 text-left">currency</th>
                <th className="py-4 px-6 text-left">Transaction Hash</th>
                <th className="py-4 px-6 text-left">from</th>
                <th className="py-4 px-6 text-left">to</th>
                <th className="py-4 px-6 text-left">Status</th>
                <th className="py-4 px-6 text-right">Amount</th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx, index) => (
                <tr 
                  key={index}
                  className="border-t border-[#2A2A2A] hover:bg-[#232323] transition-colors"
                >
                  <td className="py-4 px-6">
                    <div className="flex items-center gap-2">
                      {getIcon(tx.icon)}
                      <span className="text-gray-200">Ether</span>
                    </div>
                  </td>
                  <td className="py-4 px-6">
                    <span className="text-gray-400">{tx.id}</span>
                  </td>
                  <td className="py-4 px-6">
                    <span className="text-gray-400">{tx.from}</span>
                  </td>
                  <td className="py-4 px-6">
                    <span className="text-gray-400">{tx.to}</span>
                  </td>
                  <td className="py-4 px-6">
                    {getStatusIcon(tx.status)}
                  </td>
                  <td className="py-4 px-6">
                    <div className="text-right">
                      <div className="text-gray-200">
                        ${tx.amount.toLocaleString()}
                      </div>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
      <FloatingOverlay />
    </div>
  );
};

export default TransactionList;
