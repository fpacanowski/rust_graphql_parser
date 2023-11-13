# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("rust_graphql_parser") do |ext|
  ext.lib_dir = "lib/rust_graphql_parser"
end

task default: %i[compile spec rubocop]

task :bench do
  require 'benchmark/ips'
  require 'rust_graphql_parser'
  s = File.read('negotiate.gql')
  puts RustGraphqlParser.parse_no_copy(s)
  Benchmark.ips do |x|
    x.report('parse') { RustGraphqlParser.parse(s) }
    x.report('parse_no_copy') { RustGraphqlParser.parse_no_copy(s) }
  end
end
