#!/bin/bash

for i in {1..10000}
do
   curl -X POST http://localhost:30000/produce -d "Test message $i"
   echo "Sent message $i"
done