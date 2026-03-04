pub mod debugger;
pub mod heap_profiler;
pub mod profiler;
pub mod runtime;
pub const VERSION: &str = "1.3";
group_enum ! (Type { Debugger (debugger :: types :: Type) , HeapProfiler (heap_profiler :: types :: Type) , Profiler (profiler :: types :: Type) , Runtime (runtime :: types :: Type) });
group_enum ! (Command { Debugger (debugger :: commands :: Command) , HeapProfiler (heap_profiler :: commands :: Command) , Profiler (profiler :: commands :: Command) , Runtime (runtime :: commands :: Command) });
group_enum ! (Event { Debugger (debugger :: events :: Event) , HeapProfiler (heap_profiler :: events :: Event) , Profiler (profiler :: events :: Event) , Runtime (runtime :: events :: Event) } + other);
