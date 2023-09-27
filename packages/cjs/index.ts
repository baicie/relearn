const express = require('express')
const demo = require('./esm.mjs')

const app = express()
console.log('demo', demo)
app.listen(2)
