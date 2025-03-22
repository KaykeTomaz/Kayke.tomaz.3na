
// src/lib.rs

/// Multiplica todos os elementos de um array.
///
/// # Safety
///
/// Este método é seguro desde que o ponteiro seja válido e o comprimento seja menor ou igual ao tamanho real do array.
pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    if len == 0 {
        return 1; // Valor padrão para array vazio
    }

    let mut product = 1;
    for i in 0..len {
        product *= *ptr.offset(i as isize);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }

    #[test]
    fn test_multiply_array_empty() {
        let arr: [i32; 0] = [];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 1);
    }

    #[test]
    fn test_multiply_array_single_element() {
        let arr = [5];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 5);
    }
}