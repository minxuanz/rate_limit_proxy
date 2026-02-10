curl -X POST http://127.0.0.1:3000/post \
     -H "Content-Type: application/json" \
     -d '{"hello":"world"}'


time seq 10 | xargs -I{} -P10 curl -s http://127.0.0.1:3000/delay/3