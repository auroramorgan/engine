#![allow(non_camel_case_types)]

use std;

use buffer::{BufferView, TypedView, ScalarTypedView};

use vertex;

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
    return match *self {
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
    };
  }

  pub fn to_usize(&self) -> usize {
    return match *self {
      ScalarValue::u8(x) => x as usize,
      ScalarValue::u16(x) => x as usize,
      ScalarValue::u32(x) => x as usize,
      _ => panic!("Unsupported type conversion")
    };
  }
}

pub enum ScalarUntypedView<'a> {
  f16(ScalarTypedView<'a, i16>),
  f32(ScalarTypedView<'a, f32>),
  u8(ScalarTypedView<'a, u8>),
  u16(ScalarTypedView<'a, u16>),
  u32(ScalarTypedView<'a, u32>),
  u8_normalized(ScalarTypedView<'a, u8>),
  u16_normalized(ScalarTypedView<'a, u16>),
  u32_normalized(ScalarTypedView<'a, u32>),
  i8(ScalarTypedView<'a, i8>),
  i16(ScalarTypedView<'a, i16>),
  i32(ScalarTypedView<'a, i32>),
  i8_normalized(ScalarTypedView<'a, i8>),
  i16_normalized(ScalarTypedView<'a, i16>),
  i32_normalized(ScalarTypedView<'a, i32>)
}

impl<'a> ScalarUntypedView<'a> {
  pub fn len(&self) -> usize {
    return match *self {
      ScalarUntypedView::f16(ref x) => x.len(),
      ScalarUntypedView::f32(ref x) => x.len(),
      ScalarUntypedView::u8(ref x) => x.len(),
      ScalarUntypedView::u16(ref x) => x.len(),
      ScalarUntypedView::u32(ref x) => x.len(),
      ScalarUntypedView::u8_normalized(ref x) => x.len(),
      ScalarUntypedView::u16_normalized(ref x) => x.len(),
      ScalarUntypedView::u32_normalized(ref x) => x.len(),
      ScalarUntypedView::i8(ref x) => x.len(),
      ScalarUntypedView::i16(ref x) => x.len(),
      ScalarUntypedView::i32(ref x) => x.len(),
      ScalarUntypedView::i8_normalized(ref x) => x.len(),
      ScalarUntypedView::i16_normalized(ref x) => x.len(),
      ScalarUntypedView::i32_normalized(ref x) => x.len()
    };
  }

  pub fn get_scalar_value(&self, i: usize) -> ScalarValue {
    return match *self {
      ScalarUntypedView::f16(ref x) => ScalarValue::f16(x[i]),
      ScalarUntypedView::f32(ref x) => ScalarValue::f32(x[i]),
      ScalarUntypedView::u8(ref x) => ScalarValue::u8(x[i]),
      ScalarUntypedView::u16(ref x) => ScalarValue::u16(x[i]),
      ScalarUntypedView::u32(ref x) => ScalarValue::u32(x[i]),
      ScalarUntypedView::u8_normalized(ref x) => ScalarValue::u8_normalized(x[i]),
      ScalarUntypedView::u16_normalized(ref x) => ScalarValue::u16_normalized(x[i]),
      ScalarUntypedView::u32_normalized(ref x) => ScalarValue::u32_normalized(x[i]),
      ScalarUntypedView::i8(ref x) => ScalarValue::i8(x[i]),
      ScalarUntypedView::i16(ref x) => ScalarValue::i16(x[i]),
      ScalarUntypedView::i32(ref x) => ScalarValue::i32(x[i]),
      ScalarUntypedView::i8_normalized(ref x) => ScalarValue::i8_normalized(x[i]),
      ScalarUntypedView::i16_normalized(ref x) => ScalarValue::i16_normalized(x[i]),
      ScalarUntypedView::i32_normalized(ref x) => ScalarValue::i32_normalized(x[i])
    };
  }

  pub fn get_f32(&self, i: usize) -> f32 {
    return self.get_scalar_value(i).to_f32();
  }

  pub fn get_usize(&self, i: usize) -> usize {
    return self.get_scalar_value(i).to_usize();
  }
}

pub enum UntypedView<'a> {
  f16(TypedView<'a, i16>),
  f32(TypedView<'a, f32>),
  u8(TypedView<'a, u8>),
  u16(TypedView<'a, u16>),
  u32(TypedView<'a, u32>),
  u8_normalized(TypedView<'a, u8>),
  u16_normalized(TypedView<'a, u16>),
  u32_normalized(TypedView<'a, u32>),
  i8(TypedView<'a, i8>),
  i16(TypedView<'a, i16>),
  i32(TypedView<'a, i32>),
  i8_normalized(TypedView<'a, i8>),
  i16_normalized(TypedView<'a, i16>),
  i32_normalized(TypedView<'a, i32>)
}

impl<'a> UntypedView<'a> {
  pub fn new(view: &'a BufferView, format: vertex::Format, offset: usize, stride: usize, length: usize) -> UntypedView<'a> {
    return match format {
      vertex::Format(vertex::Scalar::f16, width) => UntypedView::f16(TypedView::<i16>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::f32, width) => UntypedView::f32(TypedView::<f32>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u8, width) => UntypedView::u8(TypedView::<u8>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u16, width) => UntypedView::u16(TypedView::<u16>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u32, width) => UntypedView::u32(TypedView::<u32>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u8_normalized, width) => UntypedView::u8_normalized(TypedView::<u8>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u16_normalized, width) => UntypedView::u16_normalized(TypedView::<u16>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::u32_normalized, width) => UntypedView::u32_normalized(TypedView::<u32>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i8, width) => UntypedView::i8(TypedView::<i8>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i16, width) => UntypedView::i16(TypedView::<i16>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i32, width) => UntypedView::i32(TypedView::<i32>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i8_normalized, width) => UntypedView::i8_normalized(TypedView::<i8>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i16_normalized, width) => UntypedView::i16_normalized(TypedView::<i16>::new(None, view, width, offset, stride, length)),
      vertex::Format(vertex::Scalar::i32_normalized, width) => UntypedView::i32_normalized(TypedView::<i32>::new(None, view, width, offset, stride, length)),
    };
  }
  
  pub fn len(&self) -> usize {
    return match *self {
      UntypedView::f16(ref x) => x.len(),
      UntypedView::f32(ref x) => x.len(),
      UntypedView::u8(ref x) => x.len(),
      UntypedView::u16(ref x) => x.len(),
      UntypedView::u32(ref x) => x.len(),
      UntypedView::u8_normalized(ref x) => x.len(),
      UntypedView::u16_normalized(ref x) => x.len(),
      UntypedView::u32_normalized(ref x) => x.len(),
      UntypedView::i8(ref x) => x.len(),
      UntypedView::i16(ref x) => x.len(),
      UntypedView::i32(ref x) => x.len(),
      UntypedView::i8_normalized(ref x) => x.len(),
      UntypedView::i16_normalized(ref x) => x.len(),
      UntypedView::i32_normalized(ref x) => x.len()
    };
  }

  pub fn get_vector_value(&self, i: usize) -> Vec<ScalarValue> {
    return match *self {
      UntypedView::f16(ref x) => x[i].iter().map(|v| ScalarValue::f16(*v)).collect(),
      UntypedView::f32(ref x) => x[i].iter().map(|v| ScalarValue::f32(*v)).collect(),
      UntypedView::u8(ref x) => x[i].iter().map(|v| ScalarValue::u8(*v)).collect(),
      UntypedView::u16(ref x) => x[i].iter().map(|v| ScalarValue::u16(*v)).collect(),
      UntypedView::u32(ref x) => x[i].iter().map(|v| ScalarValue::u32(*v)).collect(),
      UntypedView::u8_normalized(ref x) => x[i].iter().map(|v| ScalarValue::u8_normalized(*v)).collect(),
      UntypedView::u16_normalized(ref x) => x[i].iter().map(|v| ScalarValue::u16_normalized(*v)).collect(),
      UntypedView::u32_normalized(ref x) => x[i].iter().map(|v| ScalarValue::u32_normalized(*v)).collect(),
      UntypedView::i8(ref x) => x[i].iter().map(|v| ScalarValue::i8(*v)).collect(),
      UntypedView::i16(ref x) => x[i].iter().map(|v| ScalarValue::i16(*v)).collect(),
      UntypedView::i32(ref x) => x[i].iter().map(|v| ScalarValue::i32(*v)).collect(),
      UntypedView::i8_normalized(ref x) => x[i].iter().map(|v| ScalarValue::i8_normalized(*v)).collect(),
      UntypedView::i16_normalized(ref x) => x[i].iter().map(|v| ScalarValue::i16_normalized(*v)).collect(),
      UntypedView::i32_normalized(ref x) => x[i].iter().map(|v| ScalarValue::i32_normalized(*v)).collect()
    };
  }

  pub fn get_f32(&self, i: usize) -> Vec<f32> {
    return self.get_vector_value(i).iter().map(|x| x.to_f32()).collect();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use buffer::Buffer;
  use buffer::buffer_view::BufferView;
  use buffer::typed_view::{TypedView, ScalarTypedView};
  use vertex::Width;

  #[test]
  fn test_scalar_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let view = BufferView::new(None, buffer, 0, 8);
    let tv = ScalarTypedView::<u16>::new(None, &view, 2, 4, 2);
    let uv = ScalarUntypedView::u16(tv);

    assert_eq!(uv.get_f32(0), 0x0302 as f32);
    assert_eq!(uv.get_f32(1), 0x0706 as f32);
  }
  
  #[test]
  fn test_index() {
    let buffer = Buffer::new(None, None, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let view = BufferView::new(None, buffer, 0, 10);
    let tv = TypedView::<u16>::new(None, &view, Width::Vector2, 2, 4, 2);
    let uv = UntypedView::u16(tv);

    assert_eq!(uv.get_f32(0), &[0x0302 as f32, 0x0504 as f32]);
    assert_eq!(uv.get_f32(1), &[0x0706 as f32, 0x0908 as f32]);
  }
}
