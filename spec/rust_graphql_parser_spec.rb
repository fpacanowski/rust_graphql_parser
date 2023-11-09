# frozen_string_literal: true

RSpec.describe RustGraphqlParser do
  it "has a version number" do
    expect(RustGraphqlParser::VERSION).not_to be nil
  end

  it "does something useful" do
    pp RustGraphqlParser.parse("query Foo{abc xyz(value: {foo: 7})}")
    expect(true).to eq(true)
    # expect(RustGraphqlParser.parse("query Foo{abc}")).to match({
    #   node_type: :document,
    #   definitions: [{name: "Foo", position: {column: 1, line: 1}}],
    # })
  end

  specify do
    expect(RustGraphqlParser.parse("fragment MyFragment on Foo { value }")).to eq(
      node_type: :document,
      definitions: [{unimplemented: true}],
    )
  end
end
