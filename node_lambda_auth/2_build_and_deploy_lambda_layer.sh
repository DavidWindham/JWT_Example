#!/bin/bash
cd src/layers/all-dependencies/nodejs
npm install
cd ..
zip -r all-dependencies-layer.zip nodejs
mv all-dependencies-layer.zip ../../..
cd ../../..
aws lambda publish-layer-version --layer-name all-dependencies-layer --zip-file fileb://all-dependencies-layer.zip > layer-info.txt
cat layer-info.txt