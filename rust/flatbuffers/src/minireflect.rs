trait IterationVisitor {
    fn start_sequence(&mut self);
    fn end_sequence(&mut self);

    fn visit_utype(u8, &str);
    fn visit_bool(bool);
    fn visit_char(i8, &str);
    fn visit_uchar(u8, &str);
    fn visit_short(u16, &str);
    fn visit_ushort(u16, &str);
    fn visit_int(i32, &str);
    fn visit_uint(u32, &str);
    fn visit_float(f32, &str);
    fn visit_double(f64, &str);
    fn visit_string(&str);
    fn start_vector();
    fn end_vector();
    fn element(usize, ? ? , u8)
}
