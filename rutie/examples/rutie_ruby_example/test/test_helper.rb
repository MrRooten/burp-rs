$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require "rutie_ruby_example"

require "minitest/autorun"
require 'color_pound_spec_reporter'
Minitest::Reporters.use! [ColorPoundSpecReporter.new]   

