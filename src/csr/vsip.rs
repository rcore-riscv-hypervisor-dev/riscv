use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!("Vsip
580
ssip,1,1,number,Software Interrupt
stip,5,5,number,Timer Interrupt
seip,9,9,number,External Interrupt 
end
Virtual Supevisor Interrupt Pending Register.");