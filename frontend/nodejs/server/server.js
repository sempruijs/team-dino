const express = require('express');
const path = require('path');
const fs = require('fs');
const cors = require('cors');

const app = express();
const port = 8080;

// Enable CORS for cross-origin requests
app.use(cors());

// Serve static files (HTML, CSS, JS)
app.use(express.static(path.join(__dirname, '../client')));

// API endpoint to provide data for the header, footer, and booking menu

// Utility function to read a JSON file
function readJSONFile(fileName) {
    const filePath = path.join(__dirname, 'data', fileName);
    return new Promise((resolve, reject) => {
        fs.readFile(filePath, 'utf8', (err, data) => {
            if (err) {
                reject(err);
            } else {
                resolve(JSON.parse(data));
            }
        });
    });
}

// Generalized API endpoint to fetch any dataset
app.get('/api/:dataset', async (req, res) => {
    const dataset = `${req.params.dataset}.json`;
    try {
        const data = await readJSONFile(dataset);
        res.json(data);
    } catch (error) {
        console.error(`Error reading ${dataset} file:`, error);
        res.status(500).send('Server error');
    }
});

// Start the server
app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
