/**
 * DataTable Component - Accessible tabular data renderer
 */
export function renderDataTable({ columns = [], data = [], caption = '' }) {
  return `
    <div class="table-wrap">
      <table class="card-table" style="width: 100%; border-collapse: collapse; margin-top: 0.5rem;">
        ${caption ? `<caption class="visually-hidden">${caption}</caption>` : ''}
        <thead>
          <tr style="border-bottom: 2px solid var(--border); text-align: left; font-size: 0.82rem; text-transform: uppercase; color: var(--muted);">
            ${columns.map(col => `<th scope="col" style="padding: 0.6rem 0.75rem;">${col.header}</th>`).join('')}
          </tr>
        </thead>
        <tbody>
          ${data.length === 0 ? `
            <tr>
              <td colspan="${columns.length}" style="padding: 1.5rem; text-align: center; color: var(--muted);">
                No records found.
              </td>
            </tr>
          ` : data.map(row => `
            <tr style="border-bottom: 1px solid var(--border);">
              ${columns.map(col => `
                <td style="padding: 0.75rem;">
                  ${col.render ? col.render(row[col.key], row) : (row[col.key] || '—')}
                </td>
              `).join('')}
            </tr>
          `).join('')}
        </tbody>
      </table>
    </div>
  `;
}
