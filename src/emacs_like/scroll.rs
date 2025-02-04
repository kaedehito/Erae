pub fn update_scroll(cursor_y: &mut usize, screen_height: usize, scroll_offset: &mut usize) {
    if *cursor_y < *scroll_offset {
        // **カーソルが表示範囲の上に行った場合 → スクロールアップ**
        *scroll_offset = *cursor_y;
    } else if *cursor_y > *scroll_offset + screen_height - 1 {
        // **カーソルが表示範囲の下に行った場合 → スクロールダウン**
        *scroll_offset += 1;
    }

    // **カーソルをスクロール範囲内に収める**
    let max_cursor_y = *scroll_offset + screen_height - 1;
    *cursor_y = (*cursor_y).clamp(*scroll_offset, max_cursor_y);
}
