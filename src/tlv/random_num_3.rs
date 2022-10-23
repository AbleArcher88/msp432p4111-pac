# [doc = "Register `RANDOM_NUM_3` reader"] pub struct R (crate :: R < RANDOM_NUM_3_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < RANDOM_NUM_3_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < RANDOM_NUM_3_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < RANDOM_NUM_3_SPEC >) -> Self { R (reader) } } # [doc = "32-bit Random Number 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_3](index.html) module"] pub struct RANDOM_NUM_3_SPEC ; impl crate :: RegisterSpec for RANDOM_NUM_3_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [random_num_3::R](R) reader structure"] impl crate :: Readable for RANDOM_NUM_3_SPEC { type Reader = R ; } # [doc = "`reset()` method sets RANDOM_NUM_3 to value 0"] impl crate :: Resettable for RANDOM_NUM_3_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }