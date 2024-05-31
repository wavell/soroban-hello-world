import express from 'express';
import { exec } from 'child_process';

const app = express();

// Middleware to parse JSON and URL-encoded bodies
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post('/invoke', (req, res) => {
  const { id, sourceAccount, value, key } = req.body;

	console.log("executing post");
  if (!id || !sourceAccount || !value || !key) {
    return res.status(400).send('Missing required parameters');
  }

  const command = `soroban contract invoke \
    --id ${id} \
    --network testnet \
    --source "${sourceAccount}" \
    -- store_value --value "${value}" --key "${key}"`;

  exec(command, (error, stdout, stderr) => {
    if (error) {
      console.error(`Error: ${error.message}`);
      return res.status(500).send(`Error: ${error.message}`);
    }
    if (stderr) {
      console.error(`Stderr: ${stderr}`);
      return res.status(500).send(`Stderr: ${stderr}`);
    }
    res.send(`Success: ${stdout}`);
  });
});

app.listen(3000, () => {
  console.log('Server running on port 3000');
});

