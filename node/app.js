const { anime4k } = require('../pkg/ssvm_nodejs_starter_lib.js');
const express = require('express');
const fileUpload = require('express-fileupload');

const PORT = process.env.PORT || 3000;

const app = express();

app.use(fileUpload());
app.use(express.static(__dirname + '/public'));

app.post('/upload', (req, res) => {
  const buf = Uint8Array.from(req.files.image.data);
  res.set('Content-Type', 'text/png');
  res.end(Buffer.from(anime4k(buf)));
});

app.listen(PORT, () => {
  console.log(`Server running at http://127.0.0.1:${PORT}/`);
});
