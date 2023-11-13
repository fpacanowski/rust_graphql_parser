require 'spec_helper'
require 'rust_graphql_parser'
require 'benchmark/ips'

describe 'Parsing' do
  let(:query) { 'query Foo {abc}' }
  xspecify do
    pp GraphQL.parse(query)
    pp RustGraphqlParser.parse(query)
    pp translate(RustGraphqlParser.parse(query))
    ruby_ast = GraphQL.parse(query)
    rust_ast = translate(RustGraphqlParser.parse(query))
    puts "-"*80
    pp ruby_ast
    pp rust_ast
    expect(rust_ast).to eq(ruby_ast)
    expect(true).to eq(true)
  end
  
  # query_nameless_vars
  %w[
    minimal
    minimal_query
    named_query
    nested_selection
    fragment
    inline_fragment
    fragment_spread
    query_vars
    query_nameless_vars
    query_arguments
    field_arguments
    types
    minimal_mutation
    mutation_nameless_vars
  ].each do |filename|
    specify filename do
      source = File.read("spec/data/#{filename}.graphql")
      ruby_ast = GraphQL.parse(source)
      # pp ruby_ast
      # pp RustGraphqlParser.parse(source)
      rust_ast = RustGraphqlParser.parse(source)
      # pp rust_ast
      rust_ast = RustGraphqlParser.translate(rust_ast)
      # pp rust_ast
      expect(rust_ast).to eq(ruby_ast)
    end
  end

  let(:big_query) {File.read('negotiate.gql')}

  xspecify 'bench' do
    Benchmark.ips do |x|
      pp RustGraphqlParser.parse(query)
      x.report("ruby") {GraphQL.parse(big_query)}
      # x.report("rust") {RustGraphqlParser.parse(big_query)}
      x.report("rust") {translate(RustGraphqlParser.parse(big_query))}
      x.compare!
    end
  end
end
