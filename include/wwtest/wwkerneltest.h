#ifndef WW_KERNEL_TEST_H_
#define WW_KERNEL_TEST_H_

#include <linux/printk.h>

static inline void ww_kernel_test(void){
  printk("Hello, Welcome to Rust Kenel World");
}


#endif 
