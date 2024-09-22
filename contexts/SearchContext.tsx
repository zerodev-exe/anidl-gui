import React, { createContext, useContext, useState } from 'react';

interface SearchContextType {
    searchTerm: string;
    setSearchTerm: (term: string) => void;
    filterDub: boolean;
    setFilterDub: (value: boolean) => void;
    filterSub: boolean;
    setFilterSub: (value: boolean) => void;
}

const SearchContext = createContext<SearchContextType | undefined>(undefined);

export const SearchProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [searchTerm, setSearchTerm] = useState('');
    const [filterDub, setFilterDub] = useState(false);
    const [filterSub, setFilterSub] = useState(false);

    return (
        <SearchContext.Provider value={{ searchTerm, setSearchTerm, filterDub, setFilterDub, filterSub, setFilterSub }}>
            {children}
        </SearchContext.Provider>
    );
};

export const useSearch = () => {
    const context = useContext(SearchContext);
    if (!context) {
        throw new Error('useSearch must be used within a SearchProvider');
    }
    return context;
};
