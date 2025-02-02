<p align="center">
    <img alt="" height="200" src="./docs/src/assets/graphqxl-name.svg">
    <img alt="" height="200" src="./docs/src/assets/graphqxl.svg">
</p>

[![Coverage Status](https://coveralls.io/repos/github/gabotechs/graphqxl/badge.svg?branch=main)](https://coveralls.io/github/gabotechs/graphqxl?branch=main)
[![Book](https://img.shields.io/badge/book-WIP-4d76ae.svg)](https://gabotechs.github.io/graphqxl)
![](https://img.shields.io/github/v/release/gabotechs/graphqxl?color=%e535abff)

GraphQXL is a new language built on top of the GraphQL syntax that extends the original 
language with some additional features useful for creating scalable and big server side schemas.

# Documentation

There is a WIP version of the `GraphQXL book` with some useful docs, you can check it [here](https://gabotechs.github.io/graphqxl)

# Features
### Object inheritance

Use the spread operator to inherit fields from other types or inputs. Descriptions
will also be inherited.

[Try it yourself!](https://graphqxl-explorer.vercel.app?code=dHlwZSBfT3RoZXJUeXBlIHsKICAgICJEZXNjcmlwdGlvbnMgYXJlIGFsc28gaW5oZXJpdGVkIgogICAgYmFyOiBJbnQhCn0KCnR5cGUgTXlUeXBlIHsKICAgIGZvbzogU3RyaW5nIQogICAgLi4uX090aGVyVHlwZQp9)
<table>
    <thead>
        <tr>
            <th>
                Source GraphQXL
            </th>
            <th>
                Compiled GraphQL
            </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> 


```graphql
type _OtherType {
    "Descriptions are also inherited"
    bar: Int!
}

type MyType {
    foo: String!
    ..._OtherType
}
                              #GraphQXL
```
</td><td>

```graphql
type MyType {
    foo: String!
    "Descriptions are also inherited"
    bar: Int!
}




                               #GraphQL
```
</td></tr></tbody></table>

### Generics

Declare generic types and inputs in order to reuse common structures across your schema.

[Try it yourself!](https://graphqxl-explorer.vercel.app?code=dHlwZSBHZW5lcmljPFQ%2bIHsKICAgIGZvbzogVAp9Cgp0eXBlIE15U3RyaW5nVHlwZSA9IEdlbmVyaWM8U3RyaW5nIT4KCnR5cGUgTXlJbnRUeXBlID0gR2VuZXJpYzxJbnQhPgo=)
<table>
    <thead>
        <tr>
            <th>
                Source GraphQXL
            </th>
            <th>
                Compiled GraphQL
            </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> 

```graphql
type Generic<T> {
    foo: T
}

type MyStringType = Generic<String!>

type MyIntType = Generic<Int!>

                              #GraphQXL
```
</td><td>

```graphql
type MyStringType {
    foo: String!
}

type MyIntType {
    foo: Int!
}

                               #GraphQL
```
</td></tr></tbody></table>

### Modifiers

Modify `types` and `inputs` with built-in modifiers.

[Try it yourself!](https://graphqxl-explorer.vercel.app?code=dHlwZSBfTXlUeXBlIHsKICAgIGZvbzogU3RyaW5nCiAgICBiYXI6IFN0cmluZyEKfQoKdHlwZSBNeVR5cGVSZXF1aXJlZCA9IFJlcXVpcmVkPF9NeVR5cGU%2bCgp0eXBlIE15VHlwZU9wdGlvbmFsID0gT3B0aW9uYWw8X015VHlwZT4K)
<table>
    <thead>
        <tr>
            <th>
                Source GraphQXL
            </th>
            <th>
                Compiled GraphQL
            </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> 

```graphql
type _MyType {
    foo: String
    bar: String!
}

type MyTypeRequired = Required<_MyType>

type MyTypeOptional = Optional<_MyType>

                            #GraphQXL
```
</td><td>

```graphql
type MyTypeRequired {
    foo: String!
    bar: String!
}

type MyTypeOptional {
    foo: String
    bar: String
}
                               #GraphQL
```
</td></tr></tbody></table>


### Import statements

Import other `.graphqxl` files and use their definitions in the current file.

<table>
    <thead>
        <tr>
            <th>
                Source GraphQXL
            </th>
            <th>
                Compiled GraphQL
            </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> 

```graphql
# my_file.graphqxl
import "other_file"

type MyType {
    foo: OtherType!
}
                              #GraphQXL
```
```graphql
# other_file.graphqxl
type OtherType {
    bar: Int!
}
                              #GraphQXL
```
</td><td>

```graphql
# my_file.graphql
type OtherType {
    bar: Int!
}

type MyType {
    foo: OtherType!
}






                               #GraphQL
```
</td></tr></tbody></table>


## Install

There is built-in support for this programming languages:
- [Node](https://github.com/gabotechs/node-graphqxl)

There are also precompiled binaries for each architecture that you can download directly from
GitHub releases

Mac M1

```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-aarch64-apple-darwin.tar.gz
tar -xvf graphqxl-aarch64-apple-darwin.tar.gz
```

Mac Intel

```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-x86_64-apple-darwin.tar.gz
tar -xvf graphqxl-x86_64-apple-darwin.tar.gz
```

Linux x86_64

```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-x86_64-unknown-linux-gnu.tar.gz
tar -xvf graphqxl-x86_64-unknown-linux-gnu.tar.gz
```

Linux aarch64

```shell
wget https://github.com/gabotechs/graphqxl/releases/latest/download/graphqxl-aarch64-unknown-linux-gnu.tar.gz
tar -xvf graphqxl-aarch64-unknown-linux-gnu.tar.gz
```

## Usage

```shell
./graphqxl foo.graphqxl
```

this will output `foo.graphql` as a result
