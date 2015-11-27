#!/bin/bash
echo "Native from Ruby"
`time ruby pure_ruby.rb | grep "user"`

echo
echo
echo "Pure Ruby"
`time ruby embed.rb | grep "user"`