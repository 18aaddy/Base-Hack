import React from 'react';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  ResponsiveContainer,
} from 'recharts';

const GradientLineChart = () => {
  // Sample data - replace with your actual data
  const data = [
    { date: '14 Oct', value: 620 },
    { date: '15 Oct', value: 650 },
    { date: '16 Oct', value: 620 },
    { date: '17 Oct', value: 600 },
    { date: '18 Oct', value: 620 },
    { date: '19 Oct', value: 590 },
    { date: '20 Oct', value: 650 },
    { date: '21 Oct', value: 680 },
  ];

  return (
    <div className="w-full h-64 p-4">
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart
          data={data}
          margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
        >
          <defs>
            <linearGradient id="gradientFill" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="rgb(128, 128, 128)" stopOpacity={0.3} />
              <stop offset="100%" stopColor="rgb(128, 128, 128)" stopOpacity={0.1} />
            </linearGradient>
          </defs>
          {/* <CartesianGrid strokeDasharray="3 3" stroke="#333" vertical={false} /> */}
          <XAxis
            dataKey="date"
            axisLine={false}
            tickLine={false}
            tick={{ fill: '#888', fontSize: 12 }}
          />
          <YAxis
            hide={true}
            domain={['auto', 'auto']}
          />
          <Area
            type="monotone"
            dataKey="value"
            stroke="#fff"
            strokeWidth={2}
            fill="url(#gradientFill)"
            fillOpacity={1}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
};

export default GradientLineChart;