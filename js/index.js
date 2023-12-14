async function main() {
  const { KdTreeWasm3, init_panic_hook } = await import('../pkg/index.js');
  init_panic_hook();

  // Build the KdTree
  tree = new KdTreeWasm3(new Float64Array([3.0, 4.0, 5.0, 6.0, 7.0, 9.5]));
  console.log(
    // Find the closest points
    tree.best_n_within(
      // to this point
      new Float64Array([6.0, 6.0, 6.0]),
      // in 50.0 range distance
      50.0,
      // The closest 2 points
      2
    )
  );
}

main();
