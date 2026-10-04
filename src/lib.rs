//! tuack-ng 示例插件：三个组件各打一条 `hello world` 日志。
#![no_main]

use std::path::PathBuf;

use tuack_plugin_sdk::log::info;
use tuack_plugin_sdk::{
    Document, DumpDocument, Dumper, Error, OutputFile, Processor, ProcessorOutput, RenderDocument,
    Renderer, dumper, processor, renderer,
};

/// 处理器组件：记录一条日志，题目 AST 原样返回。
struct ExampleProcessor;

impl Processor for ExampleProcessor {
    fn new() -> Self {
        ExampleProcessor
    }

    fn process(&self, doc: Document) -> Result<ProcessorOutput, Error> {
        info!("hello world");
        Ok(ProcessorOutput {
            ast: doc,
            warnings: Vec::new(),
        })
    }
}

/// 渲染器组件：记录一条日志，返回一个占位主产物路径。
struct ExampleRenderer;

impl Renderer for ExampleRenderer {
    fn new() -> Self {
        ExampleRenderer
    }

    fn render(&self, _doc: RenderDocument) -> Result<(PathBuf, Vec<OutputFile>), Error> {
        info!("hello world");
        Ok((PathBuf::from("example.md"), Vec::new()))
    }
}

/// 导出器组件：记录一条日志，不产出文件。
struct ExampleDumper;

impl Dumper for ExampleDumper {
    fn new() -> Self {
        ExampleDumper
    }

    fn dump(&self, _doc: DumpDocument) -> Result<(Vec<OutputFile>, Vec<String>), Error> {
        info!("hello world");
        Ok((Vec::new(), Vec::new()))
    }
}

// 导出名必须与 plugin.toml 里各组件的 `func` 字段一致。
processor!(ExampleProcessor, process_example);
renderer!(ExampleRenderer, render_example);
dumper!(ExampleDumper, dump_example);
