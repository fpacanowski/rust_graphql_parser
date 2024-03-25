require 'benchmark/ips'
require 'rust_graphql_parser'
require 'graphql/c_parser'
require 'graphql'

source = File.read('negotiate.gql')
GC.disable
r = Benchmark.ips do |x|
  x.report('parse_raw') { RustGraphqlParser.parse_raw(source) }
  x.report('parse_ruby') { GraphQL.parse(source) }
  x.report('parse_and_translate') { RustGraphqlParser.translate(RustGraphqlParser.parse(source)) }
  x.report('parse') { RustGraphqlParser.parse(source) }
  x.report('parse2') { AnotherParser.parse(source) }

  x.compare!
end

r.data.each do |entry|
  iterations = entry.fetch(:iterations)
  time = entry.fetch(:microseconds)
  puts "#{entry.fetch(:name)} = #{format("%.2f", time/iterations/1000)} ms"
end
