## Simple Rust CLI chat with Mistral AI

```shell
 ✗ export MISTRAL_API_KEY=XXX  # free from https://console.mistral.ai/api-keys
 ✗  # building
 ✗ cargo r --release
~...~
 ✗  # testing
 ✗ ./target/release/mistral-chat "In Rust what is the difference between Fn, FnMut, and FnOnce?"  # simple prompt as a CLI param
~...~
 ✗ cat my_question.md | ./target/release/mistral-chat  # prompt as a stdin pipe, e. g. from Markdown file with your code and detailed question
~...~
 ✗ code2prompt . -t ./my-simple-prompt.hbs -O my-project-$(date +%y%m%d%H%M).md  # using code2prompt for generating basis of my project
~...~
 ✗
```
