# How to call this API

## Examples

Using `httpie`:
```
https POST https://your_gateway.com/v1/validate address=some_mail_address@outlook.com
```

Using `curl`:
```
curl -X POST https://your_gateway.com/v1/validate --data address=some_mail_address@outlook.com
curl -X POST --url https://your_gateway.com/v1/validate --header "content-type: application/json" \
  --data '{"address": "some_mail_address@outlook.com"}'
```

Example response:
```
{
    "address": "some_mail_address@outlook.com",
    "domain": "outlook.com",
    "is_valid_syntax": true,
    "username": "some_mail_address"
}
```