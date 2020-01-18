describe('RChess', function() {
    it('is launched properly', function() {
        cy.visit('http://localhost:8080')

        cy.get('.board-b72b1').should('be.visible')
        cy.get('.board-b72b1').find('img').should('have.length', 32)
    })
})
