macro_rules! impl_tag2 {
    (
        {$(
            #[$m1:meta] $x:ident($i:literal);
            #[$m2:meta] $set_x:ident();
            #[$m3:meta] $is_set:tt();
        )*}
    ) => {
        impl Tag {
            //const L: u32 = 31;

            $(
                #[$m1]
                pub fn $x(&self) -> bool {
                    get_style!(self.style, $i * 2)
                }

                #[$m2]
                pub fn $set_x(&mut self, value: bool) {
                    set_style!(self.style, ($i * 2) + 1, true);
                    set_style!(self.style, $i * 2, value);
                }

                #[$m3]
                pub fn $is_set(&self) -> bool {
                    get_style!(self.style, ($i * 2) + 1)
                }
            )*
        }
    };
}
