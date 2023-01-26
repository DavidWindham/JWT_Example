#!/bin/bash
set -eo pipefail
cd src
npm install
npm run build
cd ..
ARTIFACT_BUCKET=$(cat bucket-name.txt)
aws cloudformation package --template-file template.yml --s3-bucket $ARTIFACT_BUCKET --output-template-file out.yml
aws cloudformation deploy --template-file out.yml --stack-name dwin-node-typescript-authentication-api --capabilities CAPABILITY_NAMED_IAM
