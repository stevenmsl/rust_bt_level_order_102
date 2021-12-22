use rust_bt_level_order_102::Solution;

fn main() {
    let tree = Solution::test_fixture_1();
    println!("{:?}", Solution::level_order(tree));
}
