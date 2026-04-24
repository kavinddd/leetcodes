function generate(numRows: number): number[][] {
  if (numRows <= 0) return [];

  const rows = [[1]];

  for (let numRow = 2; numRow <= numRows; numRow++) {
    const prevRow = rows[numRow - 2];

    const row: number[] = [];
    console.log(prevRow);
    for (let i = 0; i < numRow; i++) {
      if (i === 0 || i === numRow - 1) {
        row.push(1);
      } else {
        row.push(prevRow[i - 1] + prevRow[i]);
      }
    }
    rows.push(row);
  }

  return rows;
}

function main() {
  console.log(generate(5));
}

main();
