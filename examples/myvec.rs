use smallvec_wrapper::smallvec_wrapper;

smallvec_wrapper!(
  /// A vec holds the first 5 elements on the stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  pub MyVec<T>([T; 5]);
);

fn main() {}
