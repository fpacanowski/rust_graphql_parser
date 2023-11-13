require 'spec_helper'
require "graphql/c_parser"
require 'graphql'
require 'rust_graphql_parser'
require 'benchmark/ips'

include GraphQL::Language::Nodes

def translate(node)
  case node.fetch(:node_type)
  when :document
    Document.new(definitions: node.fetch(:definitions).map{|x| translate(x)})
  when :query
    selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
    variables = node.fetch(:variable_definitions).map{|x| translate(x)}
    OperationDefinition.new(operation_type: "query", name: node[:name], selections:, variables:)
  when :selection_set
    selections = node.fetch(:items).map{|x| translate(x)}
    OperationDefinition.new(operation_type: "query", name: node[:name], selections:, variables: [])
  when :fragment_definition
    selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
    type_name = node.fetch(:type_condition).fetch(:on)
    FragmentDefinition.new(
      name: node.fetch(:name),
      line: node.fetch(:position).fetch(:line),
      col: node.fetch(:position).fetch(:column),
      type: TypeName.new(name: type_name),
      selections: selections,
    )
  when :field
    selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
    Field.new(
      name: node.fetch(:name),
      line: node.fetch(:position).fetch(:line),
      col: node.fetch(:position).fetch(:column),
      selections: selections,
    )
  when :fragment_spread
    FragmentSpread.new(
      name: node.fetch(:fragment_name),
      line: node.fetch(:position).fetch(:line),
      col: node.fetch(:position).fetch(:column),
    )
  when :inline_fragment
    selections = node.fetch(:selection_set).fetch(:items).map{|x| translate(x)}
    type_name = node.fetch(:type_condition).fetch(:on)
    InlineFragment.new(
      line: node.fetch(:position).fetch(:line),
      col: node.fetch(:position).fetch(:column),
      type: TypeName.new(name: type_name),
      selections: selections,
    )
  when :variable_definition
    VariableDefinition.new(
      name: node.fetch(:name),
      type: TypeName.new(name: node.fetch(:var_type).fetch(:name)),
    )
  end
end

describe 'Something' do
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
  ].each do |filename|
    specify filename do
      source = File.read("spec/data/#{filename}.graphql")
      ruby_ast = GraphQL.parse(source)
      # pp ruby_ast
      # pp RustGraphqlParser.parse(source)
      rust_ast = RustGraphqlParser.parse(source)
      # pp rust_ast
      rust_ast = translate(rust_ast)
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
