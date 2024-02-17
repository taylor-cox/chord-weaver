# Website
Simple npm http-server website.

## CORS Issues
### Firefox Browser:
Attempting to run the website locally with the Firefox browser, it automatically
blocks the request due to a CORS issue. To fix this, follow the steps [here](https://stackoverflow.com/a/75475676).

In addition, set `security.fileuri.strict_origin_policy` to `false` in `about:config`.