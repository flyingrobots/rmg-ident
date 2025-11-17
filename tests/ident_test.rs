use rmg_ident::{make_edge_id, make_node_id, make_type_id};

#[test]
fn domain_separation_prevents_cross_type_collisions() {
    let lbl = "foo";
    let t = make_type_id(lbl).0;
    let n = make_node_id(lbl).0;
    let e = make_edge_id(lbl).0;
    assert_ne!(t, n);
    assert_ne!(t, e);
    assert_ne!(n, e);
}
