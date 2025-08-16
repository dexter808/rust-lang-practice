mod direct_approach;
mod using_tuple;
mod using_structs;
mod using_struct_method;

fn main() {
    direct_approach::direct_approach();

    // We wanted to relate height and width together in a single data structure
    using_tuple::using_tuple();

    // tuples doesn't allow us to use named parameters, so we use struct which gives structure and
    // allows naming thus making more readable and more manageable
    using_structs::using_structs();

    // We want the function to be associated to the struct rectangle and we want only rectangle
    // to use the area method we are defining
    using_struct_method::using_struct_method();
}