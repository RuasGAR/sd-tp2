use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire,Release,Relaxed};
use std::ops::{Deref, DerefMut};

// CONSIDERAÇÕES IMPORTANTES
/* 
    Tentamos exaustivamente fazer com que o Rust nos deixasse anular a sua condição de 
    propriedade de memória, mas infelizmente sem sucesso. 

    Isso é importante porque vai nos levar a uma implementação levemente 
    diferente do que estamos acostumados. Ao invés de criar um Lock que só indica a disponibilidade
    de um valor, ele será a própria estrutura de dados que guardará os valores protegidos.

 */


pub struct Lock<T> {
    locked:AtomicBool,
    value: UnsafeCell<T> 
}

/* Implementa a trait de Send para o nosso valor, que basicamente permite referência compartilhada */
unsafe impl<T> Sync for Lock<T> where T: Send {}

impl<T> Lock<T> {
    pub const fn new(value: T)->Self {
        Self { 
            locked:AtomicBool::new(false),
            value: UnsafeCell::new(value) 
        }
    }

    pub fn acquire<'a>(&'a self) -> LockGuard<T> {
        while self.locked.compare_exchange(false,true,Acquire,Relaxed).is_ok() {
            std::hint::spin_loop();
        }
        LockGuard { lock: self }
    }

    pub fn release(&self){
        self.locked.store(false, Release);
    }
}

// GUARD REFERENTE
pub struct LockGuard<'a, T> {
    lock: &'a Lock<T>
}

impl<T> Deref for LockGuard<'_,T> {
    type Target = T;
    fn deref(&self) -> &T  {
        unsafe {&*self.lock.value.get()}
    }
}

impl<T> DerefMut for LockGuard<'_,T> {
    fn deref_mut(&mut self) -> &mut T  {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for LockGuard<'_,T> {
    fn drop(&mut self) {
        self.lock.locked.store(false,Release);
    }
}
