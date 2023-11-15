require 'benchmark/ips'
require 'rust_graphql_parser'
require 'graphql/c_parser'
require 'graphql'

source = File.read('negotiate.gql')
Benchmark.ips do |x|
  x.report('parse') { RustGraphqlParser.parse(source) }
  x.report('parse_raw') { RustGraphqlParser.parse_raw(source) }
  x.report('parse_ruby') { GraphQL.parse(source) }
end
