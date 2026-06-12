#!/usr/bin/env ruby

require 'anystyle'

# read from stdin
input = STDIN.read
exit if input.nil? || input.strip.empty?

# parse with wapiti model
dataset = AnyStyle.parse(input, format: :wapiti)

# output CSL JSON
require 'json'
puts JSON.pretty_generate(AnyStyle.parser.format_csl(dataset))
