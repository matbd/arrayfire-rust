(function() {var implementors = {};
implementors["arrayfire"] = [{"text":"impl&lt;A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;B&gt;&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'a, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;&amp;'a <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;B&gt;&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'a, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;B&gt;&gt; for &amp;'a <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'a, 'b, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;&amp;'a <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;B&gt;&gt; for &amp;'b <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;B&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'f, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; for &amp;'f <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'f, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; for &amp;'f <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'f, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for &amp;'f <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]},{"text":"impl&lt;'f, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html\" title=\"trait core::ops::bit::Shl\">Shl</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; for &amp;'f <a class=\"struct\" href=\"arrayfire/struct.Array.html\" title=\"struct arrayfire::Array\">Array</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a> + <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as <a class=\"trait\" href=\"arrayfire/trait.ImplicitPromote.html\" title=\"trait arrayfire::ImplicitPromote\">ImplicitPromote</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;&gt;::<a class=\"type\" href=\"arrayfire/trait.ImplicitPromote.html#associatedtype.Output\" title=\"type arrayfire::ImplicitPromote::Output\">Output</a>: <a class=\"trait\" href=\"arrayfire/trait.HasAfEnum.html\" title=\"trait arrayfire::HasAfEnum\">HasAfEnum</a>,&nbsp;</span>","synthetic":false,"types":["arrayfire::core::array::Array"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()