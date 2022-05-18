// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risingwave_common::types::{DataType, Datum, ScalarImpl};
use risingwave_pb::expr::expr_node::RexNode;

use super::Expr;
use crate::expr::ExprType;
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Literal {
    data: Datum,
    data_type: DataType,
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("Literal")
                .field("data", &self.data)
                .field("data_type", &self.data_type)
                .finish()
        } else {
            match &self.data {
                None => write!(f, "null"),
                // Add single quotation marks for string and interval literals
                Some(ScalarImpl::Utf8(v)) => write!(f, "'{}'", v),
                Some(ScalarImpl::Interval(v)) => write!(f, "'{}'", v),
                Some(v) => write!(f, "{}", v),
            }?;
            write!(f, ":{:?}", self.data_type)
        }
    }
}

impl Literal {
    pub fn new(data: Datum, data_type: DataType) -> Self {
        Literal { data, data_type }
    }

    pub fn get_expr_type(&self) -> ExprType {
        ExprType::ConstantValue
    }

    pub fn get_data(&self) -> &Datum {
        &self.data
    }
}

impl Expr for Literal {
    fn return_type(&self) -> DataType {
        self.data_type.clone()
    }

    fn to_expr_proto(&self) -> risingwave_pb::expr::ExprNode {
        use risingwave_pb::expr::*;
        ExprNode {
            expr_type: self.get_expr_type() as i32,
            return_type: Some(self.return_type().to_protobuf()),
            rex_node: literal_to_protobuf(self.get_data()),
        }
    }
}

/// Convert a literal value (datum) into protobuf.
fn literal_to_protobuf(d: &Datum) -> Option<RexNode> {
    let Some(d) = d.as_ref() else {
        return None;
    };
    use risingwave_pb::expr::*;
    let body = ScalarImpl::to_protobuf(&Some(d.clone()));
    Some(RexNode::Constant(ConstantValue { body }))
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use risingwave_common::array::StructValue;
    use risingwave_common::types::ScalarImpl;
    use risingwave_pb::expr::expr_node::RexNode;
    use risingwave_pb::expr::StructValue as ProstStructValue;

    use crate::expr::literal::literal_to_protobuf;

    #[test]
    fn test_literal_to_protobuf() {
        let value = StructValue::new(vec![
            Some(ScalarImpl::Utf8("12222".to_string())),
            Some(ScalarImpl::Int32(2)),
            Some(ScalarImpl::Int32(3)),
        ]);
        let data = Some(ScalarImpl::Struct(value));
        let node = literal_to_protobuf(&data);
        if let RexNode::Constant(c) = node.as_ref().unwrap() {
            let decoded: ProstStructValue = Message::decode(&c.body[..]).unwrap();
            let prost = ProstStructValue {
                body: vec![vec![49, 50, 50, 50, 50], vec![0, 0, 0, 2], vec![0, 0, 0, 3]],
            };
            assert_eq!(decoded, prost);
        }
    }
}
