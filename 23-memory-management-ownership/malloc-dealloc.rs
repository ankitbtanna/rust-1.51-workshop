fn get_final_orders() -> i64 {
    let mut total_orders = 0;

    {
        let orders = vec![1, 2, 3, 4]; // alloc

        for order in orders.iter() {
           total_orders += order;
        }
        //dealloc(orders)
    }

    let final_orders = finish(total_orders);
    return final_orders;
}

finish(total) {
    // some logic here
    return total;
}

get_final_orders();