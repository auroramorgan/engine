#![allow(non_camel_case_types)]

use std;

use buffer::TypedView;

extern {
  #[link_name = "llvm.convert.from.fp16.f32"]
  pub fn convert_from_fp16_to_f32(a: i16) -> f32;
}

pub enum ScalarValue {
  f16(i16), f32(f32),
  u8(u8), u16(u16), u32(u32),
  u8_normalized(u8), u16_normalized(u16), u32_normalized(u32),
  i8(i8), i16(i16), i32(i32),
  i8_normalized(i8), i16_normalized(i16), i32_normalized(i32)
}

impl ScalarValue {
  pub fn to_f32(&self) -> f32 {
    match *self {
      ScalarValue::f16(x) => unsafe { convert_from_fp16_to_f32(x) },
      ScalarValue::f32(x) => x,
      ScalarValue::u8(x) => x as f32,
      ScalarValue::u16(x) => x as f32,
      ScalarValue::u32(x) => x as f32,
      ScalarValue::u8_normalized(x) => x as f32 / std::u8::MAX as f32,
      ScalarValue::u16_normalized(x) => x as f32 / std::u16::MAX as f32,
      ScalarValue::u32_normalized(x) => x as f32 / std::u32::MAX as f32,
      ScalarValue::i8(x) => x as f32,
      ScalarValue::i16(x) => x as f32,
      ScalarValue::i32(x) => x as f32,
      ScalarValue::i8_normalized(x) => (x as f32 / std::i8::MAX as f32).max(-1.0),
      ScalarValue::i16_normalized(x) => (x as f32 / std::i16::MAX as f32).max(-1.0),
      ScalarValue::i32_normalized(x) => (x as f32 / std::i32::MAX as f32).max(-1.0)
    }
  }
}

pub enum UntypedView<'a> {
  f16(TypedView<'a, i16>), f32(TypedView<'a, f32>),
  u8(TypedView<'a, u8>), u16(TypedView<'a, u16>), u32(TypedView<'a, u32>),
  u8_normalized(TypedView<'a, u8>), u16_normalized(TypedView<'a, u16>), u32_normalized(TypedView<'a, u32>),
  i8(TypedView<'a, i8>), i16(TypedView<'a, i16>), i32(TypedView<'a, i32>),
  i8_normalized(TypedView<'a, i8>), i16_normalized(TypedView<'a, i16>), i32_normalized(TypedView<'a, i32>)
}

impl<'a> UntypedView<'a> {
  pub fn get_scalar_value(&self, i: usize) -> ScalarValue {
    return match *self {
      UntypedView::f16(ref x) => ScalarValue::f16(x[i]),
      UntypedView::f32(ref x) => ScalarValue::f32(x[i]),
      UntypedView::u8(ref x) => ScalarValue::u8(x[i]),
      UntypedView::u16(ref x) => ScalarValue::u16(x[i]),
      UntypedView::u32(ref x) => ScalarValue::u32(x[i]),
      UntypedView::u8_normalized(ref x) => ScalarValue::u8_normalized(x[i]),
      UntypedView::u16_normalized(ref x) => ScalarValue::u16_normalized(x[i]),
      UntypedView::u32_normalized(ref x) => ScalarValue::u32_normalized(x[i]),
      UntypedView::i8(ref x) => ScalarValue::i8(x[i]),
      UntypedView::i16(ref x) => ScalarValue::i16(x[i]),
      UntypedView::i32(ref x) => ScalarValue::i32(x[i]),
      UntypedView::i8_normalized(ref x) => ScalarValue::i8_normalized(x[i]),
      UntypedView::i16_normalized(ref x) => ScalarValue::i16_normalized(x[i]),
      UntypedView::i32_normalized(ref x) => ScalarValue::i32_normalized(x[i])
    };
  }

  pub fn get_f32(&self, i: usize) -> f32 {
    return self.get_scalar_value(i).to_f32();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use buffer::Buffer;
  use buffer::buffer_view::BufferView;
  use buffer::typed_view::TypedView;

  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let view = BufferView::new(None, buffer, 0, 8);
    let tv = TypedView::<u16>::new(None, &view, 2, 4, 2);

    let uv = UntypedView::u16(tv);

    assert_eq!(uv.get_f32(0), 0x0302 as f32);
    assert_eq!(uv.get_f32(1), 0x0706 as f32);
  }
}
