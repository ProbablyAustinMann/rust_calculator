extern crate itertools;

use std::ops::Deref;
use self::itertools::chain;
use parser::{Node, Op};
use std::collections::HashMap;
use std::f64::EPSILON;
use std::rc::Rc;
use std::{cmp, fmt};
use std::cell::RefCell;