raikiri run

curl http://localhost:3000 \
  --header "Content-Type: application/json" \
  --header "Platform-Command: invoke" \
  --header "Invoke-Component: hello" \
  --header "Invoke-Method: GET"
